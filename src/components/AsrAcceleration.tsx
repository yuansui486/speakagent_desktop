import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { asrAccelerationInfo, setAsrProvider, type AsrProvider } from "../lib/api";
import { useT } from "../i18n";

function cudaVersion(value: number | null | undefined) {
  return value ? `${Math.floor(value / 1000)}.${Math.floor((value % 1000) / 10)}` : null;
}
function cudnnVersion(value: number | null | undefined) {
  return value ? `${Math.floor(value / 1000)}.${Math.floor((value % 1000) / 100)}.${value % 100}` : null;
}

export default function AsrAcceleration() {
  const t = useT().acceleration;
  const { data, error, isFetching, refetch } = useQuery({
    queryKey: ["asrAcceleration"], queryFn: asrAccelerationInfo, staleTime: 15_000,
  });
  const [saving, setSaving] = useState(false);
  const [saveError, setSaveError] = useState("");
  async function choose(provider: AsrProvider) {
    setSaving(true); setSaveError("");
    try { await setAsrProvider(provider); await refetch(); }
    catch (e) { setSaveError(String(e)); }
    finally { setSaving(false); }
  }
  const runtime = data?.runtime;
  const ready = runtime && !runtime.missingLibraries.length && runtime.cudaRuntimeVersion === 11080
    && runtime.cudnnVersion !== null && runtime.cudnnVersion >= 8000 && runtime.cudnnVersion < 9000
    && runtime.sherpaVersion === "1.13.4" && !!runtime.onnxruntimeVersion;
  function reasonLabel(reason: string) {
    if (reason.includes("No NVIDIA")) return t.noGpu;
    if (reason.includes("Insufficient free VRAM")) return t.lowMemory;
    if (/runtime|DLL|supports Windows/.test(reason)) return t.unavailable;
    return t.failed;
  }
  const rows = data ? [
    [t.driverCuda, data.driverCudaVersion ?? t.unknown],
    [t.toolkit, data.toolkitVersion ?? t.absent],
    [t.runtime, cudaVersion(runtime?.cudaRuntimeVersion) ?? t.absent],
    [t.cudnn, cudnnVersion(runtime?.cudnnVersion) ?? t.absent],
    [t.ort, runtime?.onnxruntimeVersion ?? t.absent],
    [t.actual, data.lastProvider?.toUpperCase() ?? t.pending],
  ] : [];
  return <section className="glass mt-6 rounded-xl border border-white/5 p-4">
    <h2 className="font-medium text-zinc-200">{t.title}</h2>
    <p className="mt-2 text-xs leading-relaxed text-zinc-400">{t.hint}</p>
    <div className="mt-3 flex flex-wrap items-center gap-2">
      {(["auto", "cpu", "cuda"] as const).map(provider => <button key={provider}
        disabled={saving || !data} aria-pressed={data?.preference === provider}
        onClick={() => void choose(provider)}
        className={`rounded-lg border px-3 py-1.5 text-sm disabled:opacity-50 ${data?.preference === provider ? "border-amber-500/40 bg-amber-500/10 text-amber-400" : "border-white/10 text-zinc-300 hover:bg-white/5"}`}>
        {t[provider]}</button>)}
      <button disabled={isFetching || saving} onClick={() => void refetch()}
        className="ml-auto text-xs text-zinc-400 hover:text-zinc-200 disabled:opacity-50">{isFetching ? t.loading : t.refresh}</button>
    </div>
    {data?.devices.map(device => <div key={device.index} className="mt-4 text-xs leading-6 text-zinc-300">
      <p>{t.gpu}: {device.name} · SM {device.computeCapability}</p>
      <p>{t.memory}: {device.freeMemoryMb} / {device.totalMemoryMb} MB · {t.driver}: {device.driverVersion}</p>
    </div>)}
    <dl className="mt-3 grid grid-cols-2 gap-x-4 gap-y-2 text-xs">
      {rows.map(([label, value]) => <div key={label}><dt className="text-zinc-500">{label}</dt><dd className="mt-1 break-words text-zinc-300">{value}</dd></div>)}
    </dl>
    <p className="mt-3 text-xs leading-relaxed text-zinc-500">{t.distinction}</p>
    {data?.lastProvider === "cuda" && <p className="mt-2 text-xs leading-relaxed text-zinc-500">{t.hybrid}</p>}
    {data && !ready && <p className="mt-3 text-xs leading-relaxed text-amber-400">{t.install}</p>}
    {!!runtime?.missingLibraries.length && <p className="mt-2 break-words text-xs text-amber-400">{t.missing}: {runtime.missingLibraries.join(", ")}</p>}
    {data?.fallbackReason && <div className="mt-3 text-xs text-amber-400">
      <p>{t.fallback}: {reasonLabel(data.fallbackReason)}</p>
      <details className="mt-1 text-zinc-500"><summary className="cursor-pointer">{t.details}</summary><p className="mt-1 break-words">{data.fallbackReason}</p></details>
    </div>}
    {(saveError || error || data?.detectionError) && <div role="alert" className="mt-3 text-xs text-red-400">
      {t.detectionFailed}<details className="mt-1"><summary className="cursor-pointer">{t.details}</summary><p className="break-words">{saveError || String(error ?? data?.detectionError)}</p></details>
    </div>}
  </section>;
}
