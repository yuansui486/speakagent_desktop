# 中文识别与 CUDA 加速

应用识别引擎和可选 CUDA 推理进程均使用 **Rust + sherpa-onnx 1.13.4**。安装脚本使用 PowerShell；不需要 Python、pip 或 Python 服务。

## 模型选择

| 模型 | 推荐场景 | 下载大小 | 时间定位 |
| --- | --- | --- | --- |
| SenseVoiceSmall INT8（默认） | 普通话、粤语、日常语音输入；CPU 速度优先 | 约 238 MB | 中文 token 时间戳 |
| Qwen3-ASR 0.6B INT8 | 方言、复杂中文、标点；质量优先 | 约 987 MB | VAD 语音片段 |
| Whisper small / large-v3 turbo | 多语言及已有工作流的对比选择 | 约 466 / 540 MB | 语音片段 |

新用户默认简体中文和 SenseVoice；保留已有界面语言和模型选择。Parakeet 的这个检查点只支持 25 种欧洲语言，不支持中文。GigaAM 面向俄语。

不存在适用于所有音频的“最好”模型。Qwen3-ASR 发布方报告的 WenetSpeech meeting 字错误率：Whisper-large-v3 19.11%、Qwen3-ASR 0.6B 6.88%、1.7B 5.88%；AISHELL-2 为 5.06%、3.15%、2.71%。这是**发布方原始模型的评测**，不是本项目 INT8 模型的独立评测，也不能直接套用到 Whisper turbo。1.7B 对本机 2 GB 显卡负担过大，当前集成 0.6B 版本。

本机用真实官方普通话、粤语等样例验证了两个 INT8 模型。4 线程 CPU 的少量样例中，SenseVoice 解码耗时约为音频时长的 0.03–0.04 倍，Qwen 约 0.2–0.49 倍，不含首次加载、下载和说话人分离。这不是通用性能或准确率承诺。Rust 实际识别同一普通话样例：SenseVoice 为“放时间早上九点至下午五点”，Qwen 为“开放时间：早上九点至下午五点。”。Qwen 此例保留更多信息；两种模型都可能漏词或在噪声中产生错误。

中文 token、片段和说话人文本拼接不会在每个汉字之间插入空格；Qwen 使用 128 维特征和较大的生成上限，避免快速中文片段因默认生成长度被截断。首次下载固定模型修订，使用临时文件，完成后替换；缺少任一所需文件都不会显示为已安装。

## Windows CUDA 安装

在项目根目录执行：

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\setup-cuda.ps1
```

需要 Rust MSVC 工具链、Visual Studio C++ Build Tools、NVIDIA 驱动，以及支持 bzip2 的 tar 或 7-Zip。脚本下载约 1.5 GB 组件，解压后还需要额外磁盘空间。支持 `-CacheDir` 和 `-InstallDir`；自定义安装目录必须与应用实际模型目录下的 `bin/asr-cuda` 一致。

脚本从 sherpa-onnx 官方 GitHub 发布和 NVIDIA 发布的原生 DLL 分发包提取文件，校验 SHA-256，编译 Rust worker，安装到 `%LOCALAPPDATA%\SpeakAgent\models\bin\asr-cuda`。NVIDIA 分发包虽然使用 `.whl` ZIP 格式，但这里只提取原生 DLL 和许可文件，**不安装或调用 Python**。脚本不更改系统 PATH；只有子进程使用本地 DLL 路径。不要将 CUDA DLL 覆盖到主程序目录。

打开“设置 → 语音识别显卡加速”，点击“重新检测”。可分别查看显卡、可用显存、驱动、驱动 CUDA 支持上限、可选开发工具包、实际 CUDA 运行库、cuDNN、ONNX Runtime 和最近识别后端。运行库检测成功不等于某个模型一定能使用显卡；最近识别状态在实际推理后更新。

| 模式 | 行为 |
| --- | --- |
| 自动（默认） | 检查显卡、运行库、可用显存，再尝试 CUDA；不足则 CPU |
| CPU | 直接使用 CPU |
| CUDA | 即使低于自动显存门槛仍尝试 CUDA，失败后 CPU 重试 |

切换在下一次识别生效，语音输入缓存也按模式重新加载。模型在独立 Rust 子进程中保持加载；初始化、进程、超时、显存或推理错误会终止该子进程，使用 CPU 重新识别**同一音频片段**。VAD、说话人分离和文字整理没有因此改为 CUDA。

## 本机兼容性

检测到 NVIDIA GeForce GTX 1050，2 GB 显存，计算能力 6.1，驱动 581.57。驱动显示的 CUDA 13.0 是支持上限；本机未检测到 `nvcc`，不能据此认为已安装 CUDA 13 工具包。CUDA 13 已不适合 Pascal，所以安装脚本固定采用 CUDA **11.8**、cuDNN **8.9.5**、官方 sherpa CUDA 构建对应的 ONNX Runtime **1.17.1**。

这张显卡同时承担桌面显示，检测时仅约 560 MB 可用。自动模式保守要求 SenseVoice 等模型至少 768 MB、Qwen 至少 2048 MB 可用显存，因而可能选择 CPU。门槛是避免无效尝试的估计，并非模型能运行的保证。INT8 模型的部分算子可能仍在 CPU 上运行，CUDA 也可能更慢；不能仅凭安装或选中 CUDA 宣称加速倍数。不应关闭用户其他程序来强行腾出显存。

### 本机实际验证（2026-11-13）

运行组件已安装并实测：CUDA 11.8、cuDNN 8.9.5、ORT 1.17.1、sherpa-onnx 1.13.4，探针无缺失组件。使用应用实际 Rust ASR/config/runtime 模块和独立 Rust CUDA worker、4 个 CPU 线程，对同一 5.592 秒官方普通话样例测量：

| 模型 / 模式 | 首次加载 | 解码 | 实际后端 |
| --- | --- | --- | --- |
| SenseVoice / CPU | 1.60 秒 | 0.229 秒 | CPU |
| SenseVoice / 强制 CUDA | 4.32 秒 | 0.483 秒 | CUDA（可能含 CPU 算子） |
| Qwen3-ASR / CPU | 4.19 秒 | 1.211 秒 | CPU |
| Qwen3-ASR / 强制 CUDA | 7.63 秒 | 7.548 秒 | CUDA（可能含 CPU 算子） |

两种模式分别得到相同识别文本。使用 `nvidia-smi` 同步采样可观察到 CUDA 运行期间显存和 GPU 利用率变化；Qwen 期间总显存使用峰值约 1841 MB，GPU 利用率峰值约 53%。显示进程和其他应用也会改变这些数值，不能视为模型独占内存或逐算子分配证明。这些是单次短样例、非隔离桌面环境测量，**本机此配置 CUDA 更慢**；CPU 是合理选择。

自动模式在 689 MB 可用显存时正确使用 CPU，并记录 768 MB 门槛原因。额外在模型初始化后主动终止测试子进程，Rust 主调用成功将同一段音频交回 CPU，输出完整文本并记录管道故障原因。未终止任何用户应用。测试完成后无残留 CUDA worker。

发布安装包时，主应用仍可独立使用 CPU；CUDA 是可选本地组件。分发 worker 时应一并分发匹配版本的原生库及上游许可，不能只复制 EXE。

## 上游资料与许可

- [Qwen3-ASR：模型能力与评测](https://github.com/QwenLM/Qwen3-ASR)，[sherpa Qwen3-ASR 原生接入](https://k2-fsa.github.io/sherpa/onnx/qwen3-asr/pretrained.html)。Qwen 模型使用 Apache-2.0。
- [SenseVoice](https://github.com/FunAudioLLM/SenseVoice)，[sherpa SenseVoice 检查点](https://k2-fsa.github.io/sherpa/onnx/sense-voice/pretrained.html)。模型权重遵循 [FunASR 模型许可](https://huggingface.co/FunAudioLLM/SenseVoiceSmall/blob/main/LICENSE)，请按实际用途核对，不将仓库代码许可等同于权重许可。
- [FunASR](https://github.com/modelscope/FunASR) 与 [Fun-ASR](https://github.com/QwenAudio/Fun-ASR) 也提供中文识别方案；本次选择已有原生 sherpa Rust 接口的检查点。
- [sherpa-onnx 1.13.4 发布](https://github.com/k2-fsa/sherpa-onnx/releases/tag/v1.13.4)，[Windows CUDA 构建配置](https://github.com/k2-fsa/sherpa-onnx/blob/v1.13.4/.github/workflows/build-wheels-win64-cuda.yaml)，[原生会话与 provider 回退实现](https://github.com/k2-fsa/sherpa-onnx/blob/v1.13.4/sherpa-onnx/csrc/session.cc)。
- [ONNX Runtime CUDA / cuDNN 兼容表](https://onnxruntime.ai/docs/execution-providers/CUDA-ExecutionProvider.html)。cuDNN 8 与 9 不可混用；驱动、工具包、应用运行库版本应分别检测。
