param(
    [string]$InstallDir = (Join-Path $env:LOCALAPPDATA 'SpeakAgent/models/bin/asr-cuda'),
    [string]$CacheDir = (Join-Path $PSScriptRoot '../.cache/cuda'),
    [switch]$SkipBuild
)
$ErrorActionPreference = 'Stop'
if ($env:OS -ne 'Windows_NT' -or ![Environment]::Is64BitProcess) {
    throw 'This CUDA runtime installer supports Windows x64.'
}
$InstallDir = [IO.Path]::GetFullPath($InstallDir)
$CacheDir = [IO.Path]::GetFullPath($CacheDir)
New-Item -ItemType Directory -Force -Path $InstallDir, $CacheDir | Out-Null

function Get-Json([string]$Url) {
    $result = & curl.exe --fail --silent --show-error --location --retry 3 --connect-timeout 20 --max-time 90 --url $Url
    if ($LASTEXITCODE -ne 0) { throw "Metadata download failed: $Url" }
    return ($result -join "`n" | ConvertFrom-Json)
}

function Get-VerifiedFile([string]$Url, [string]$Name, [string]$Sha256) {
    $destination = Join-Path $CacheDir $Name
    if ((Test-Path -LiteralPath $destination) -and ((Get-FileHash -LiteralPath $destination -Algorithm SHA256).Hash -eq $Sha256)) {
        return $destination
    }
    $partial = "$destination.part"
    for ($attempt = 1; $attempt -le 3; $attempt++) {
        Write-Host "Downloading $Name (attempt $attempt)..."
        & curl.exe --fail --location --retry 3 --retry-all-errors --connect-timeout 20 --max-time 900 --output $partial --url $Url
        if ($LASTEXITCODE -eq 0 -and (Test-Path -LiteralPath $partial)) {
            if ((Get-FileHash -LiteralPath $partial -Algorithm SHA256).Hash -eq $Sha256) {
                Move-Item -LiteralPath $partial -Destination $destination -Force
                return $destination
            }
        }
    }
    throw "Download incomplete or SHA-256 mismatch: $Name"
}

# Pinned upstream sherpa-onnx Windows CUDA 11.8 / cuDNN 8 build. Unlike CUDA 13,
# this track supports Pascal (e.g. GTX 1050). Driver support != installed runtime.
$archiveName = 'sherpa-onnx-v1.13.4-win-x64-cuda.tar.bz2'
$archive = Get-VerifiedFile "https://github.com/k2-fsa/sherpa-onnx/releases/download/v1.13.4/${archiveName}?download=1" $archiveName '9cc16169fb073ab0acd304ae144ccad21af03e8360921a12285105599f0f692a'
$unpack = Join-Path $CacheDir 'upstream'
New-Item -ItemType Directory -Force -Path $unpack | Out-Null
& tar.exe -xjf $archive -C $unpack
if ($LASTEXITCODE -ne 0) {
    $sevenZip = Join-Path $env:ProgramFiles '7-Zip/7z.exe'
    if (!(Test-Path -LiteralPath $sevenZip)) { throw 'Install 7-Zip: this Windows tar cannot extract bzip2 archives.' }
    & $sevenZip x -y "-o$unpack" $archive | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'bzip2 extraction failed.' }
    & $sevenZip x -y "-o$unpack" (Join-Path $unpack ($archiveName -replace '\.bz2$', '')) | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'CUDA runtime archive extraction failed.' }
}
$upstream = Join-Path $unpack 'sherpa-onnx-v1.13.4-win-x64-cuda'
Get-ChildItem -LiteralPath (Join-Path $upstream 'lib') -Filter '*.dll' | Copy-Item -Destination $InstallDir -Force

# NVIDIA distributes redistributable native DLLs in ZIP-compatible wheel archives.
# They are extracted directly; Python and pip are never installed or invoked.
Add-Type -AssemblyName System.IO.Compression.FileSystem
$packages = @(
    @{ Name = 'nvidia-cuda-runtime-cu11'; Version = '11.8.89' },
    @{ Name = 'nvidia-cublas-cu11'; Version = '11.11.3.6' },
    @{ Name = 'nvidia-cufft-cu11'; Version = '10.9.0.58' },
    @{ Name = 'nvidia-cudnn-cu11'; Version = '8.9.5.29' }
)
foreach ($package in $packages) {
    $metadata = Get-Json "https://pypi.org/pypi/$($package.Name)/$($package.Version)/json"
    $asset = $metadata.urls | Where-Object filename -Match 'win_amd64\.whl$' | Select-Object -First 1
    if (!$asset) { throw "No Windows DLL package found: $($package.Name) $($package.Version)" }
    $wheel = Get-VerifiedFile "$($asset.url)?download=1" $asset.filename $asset.digests.sha256
    $zip = [IO.Compression.ZipFile]::OpenRead($wheel)
    try {
        foreach ($entry in $zip.Entries) {
            if ($entry.Name -like '*.dll') {
                [IO.Compression.ZipFileExtensions]::ExtractToFile($entry, (Join-Path $InstallDir $entry.Name), $true)
            } elseif ($entry.Name -match '(?i)license|eula') {
                $licenseDir = Join-Path $InstallDir "licenses/$($package.Name)"
                New-Item -ItemType Directory -Force -Path $licenseDir | Out-Null
                [IO.Compression.ZipFileExtensions]::ExtractToFile($entry, (Join-Path $licenseDir $entry.Name), $true)
            }
        }
    } finally { $zip.Dispose() }
}

if (!$SkipBuild) {
    $vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio/Installer/vswhere.exe'
    if (!(Test-Path -LiteralPath $vswhere)) { throw 'Visual Studio C++ Build Tools are required to build the Rust worker.' }
    $visualStudio = & $vswhere -latest -products '*' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
    $toolVersion = (Get-Content -LiteralPath (Join-Path $visualStudio 'VC/Auxiliary/Build/Microsoft.VCToolsVersion.default.txt') -Raw).Trim()
    $libTool = Join-Path $visualStudio "VC/Tools/MSVC/$toolVersion/bin/Hostx64/x64/lib.exe"
    $libDir = Join-Path $upstream 'lib'
    # The upstream archive includes the ORT DLL but not its MSVC import library.
    $definition = Join-Path $libDir 'onnxruntime.def'
    [IO.File]::WriteAllText($definition, "LIBRARY onnxruntime`nEXPORTS`nOrtGetApiBase`n", [Text.Encoding]::ASCII)
    & $libTool /nologo /machine:x64 "/def:$definition" "/out:$(Join-Path $libDir 'onnxruntime.lib')"
    if ($LASTEXITCODE -ne 0) { throw 'ONNX Runtime import library generation failed.' }
    $oldLibDir = $env:SHERPA_ONNX_LIB_DIR
    try {
        $env:SHERPA_ONNX_LIB_DIR = $libDir
        $manifest = Join-Path $PSScriptRoot '../native/asr-cuda-worker/Cargo.toml'
        & cargo build --release --locked --manifest-path $manifest
        if ($LASTEXITCODE -ne 0) { throw 'Rust CUDA worker build failed.' }
        Copy-Item -LiteralPath (Join-Path $PSScriptRoot '../native/asr-cuda-worker/target/release/speakagent-asr-cuda.exe') -Destination $InstallDir -Force
    } finally { $env:SHERPA_ONNX_LIB_DIR = $oldLibDir }
}

$worker = Join-Path $InstallDir 'speakagent-asr-cuda.exe'
if (Test-Path -LiteralPath $worker) {
    $probeOutput = & $worker --probe
    if ($LASTEXITCODE -ne 0) { throw 'CUDA runtime probe failed. Check the NVIDIA driver and local DLLs.' }
    $probe = ($probeOutput -join "`n") | ConvertFrom-Json
    if ($probe.missingLibraries.Count -gt 0 -or $probe.cudaRuntimeVersion -ne 11080 -or
        $probe.cudnnVersion -lt 8000 -or $probe.cudnnVersion -ge 9000 -or
        $probe.sherpaVersion -ne '1.13.4' -or !$probe.onnxruntimeVersion) {
        throw "CUDA runtime probe found missing or incompatible libraries: $($probeOutput -join ' ')"
    }
    $probeOutput | Write-Output
} elseif ($SkipBuild) {
    Write-Host "CUDA DLLs installed in $InstallDir. Build the Rust worker before enabling CUDA."
    return
} else {
    throw 'CUDA worker executable is missing after installation.'
}
@{ sherpaVersion = '1.13.4'; cudaMajor = 11; cudnnMajor = 8; source = 'k2-fsa/sherpa-onnx'; installedAt = [DateTime]::UtcNow.ToString('o') } |
    ConvertTo-Json | Set-Content -LiteralPath (Join-Path $InstallDir 'runtime.json') -Encoding utf8
Write-Host "CUDA runtime installed in $InstallDir. No system PATH changes were made."
