# 说话人分离模型选择与修复记录

## 本项目当前推荐

中文质量优先采用 **pyannote segmentation 3.0 + ERes2NetV2 中文声纹模型**，全部通过 sherpa-onnx 的 Rust 接口本地执行，不需要 Python。设置中保留 **CAM++ 中英多语言**快速选项。

不存在对全部录音都最优的单一模型。说话人分离包括语音分段、声纹提取、聚类和与转写文本对齐，替换声纹模型不能直接获得另一套完整系统的评测结果。

| 方案 | 依据 | 本项目适用性 |
| --- | --- | --- |
| ERes2NetV2 + pyannote 3.0 | 官方中文声纹模型卡：约 20 万说话人训练，CN-Celeb EER 为 3.81%，同表 CAM++ 为 4.32%（越低越好） | 约 72 MB 声纹模型，Rust 原生可运行；中文默认，CPU 较慢 |
| CAM++ advanced + pyannote 3.0 | 中英多语言声纹模型，约 28 MB | 保留为快速选项；上述 4.32% 是模型卡中的 CAM++ 比较项，不是本项目 advanced 转换模型的独立测量 |
| pyannote Community-1 | 官方完整系统 DER：AISHELL-4 11.7%、AliMeeting 20.3% | 开源完整流程，但需接受权重使用条件；本项目现有 Rust 接口不能靠替换一个模型文件完整复现其流程 |
| pyannote Precision-2 | 同一官方表中 AISHELL-4 11.4%、AliMeeting 15.2%，优于 Community-1 | 商业服务／商业自托管方案；本次没有接入，也没有向云端发送录音 |

**EER 是声纹验证错误率，DER 是整段录音的分离错误率，两者不可直接比较。** 上表数值来自不同发布方的报告，本次没有重跑这些完整评测集。ERes2NetV2 是本项目当前约束下的中文优先选择，不宣称是所有说话人分离系统中的第一名。

另外核对了 sherpa 支持的 Reverb diarization v1 分段模型，官方文档注明其非商业许可；当前保留已有 MIT 许可的 pyannote segmentation 3.0。

## 本机验证（2026-11-13）

- 官方 `0-four-speakers-zh.wav` 中文四人样例，16 kHz、约 56.86 秒，4 CPU 线程，自动聚类阈值 0.8，分段模型相同。CAM++ advanced 自动得到 3 人，ERes2NetV2 自动得到 4 人。这是单一样例的人数检查，未计算带标注的 DER。
- 对此前报错的本地 87 秒 MP4，通过实际 Rust 解码、pyannote + ERes2NetV2 分离、VAD 和 Qwen3-ASR 0.6B 转写完整跑通，获得非空带说话人文本。该视频没有人工时间标注，因此只验证流程可用，不作为准确率评测。
- ERes2NetV2 下载后核对官方 SHA-256：`bf1a75b9930474cf3389ef415e6e5d38ca96fea4a3a00f7e301d080a58ee2239`。
- 分离模型在 CPU 执行；ASR 的 CUDA 设置不会自动加速说话人分离。2 GB 显卡不是这套分离流程的必要条件。

已知人数时，请在转写页面指定说话人数。短促发声、重叠讲话、音乐、混响或音质差仍可能导致人数与边界错误。Qwen 的转写时间定位为 VAD 片段级，短片段中的多人换声对齐也有局限。

## 本次转写报错的原因与修复

数据库中最近失败任务的错误为“尚未安装说话人识别模型，请在设置中下载”。开启了区分说话人，但 Debug 启动时跳过了未下载的打包资源；原来的后台准备逻辑又静默忽略下载错误，导致 ASR 模型已安装却不能分离说话人。

已补齐本机分段、声纹、VAD 和解码器，并修改转写及 MCP 流程：转写前检查本次选择需要的辅助模型，缺失时先下载并显示准备进度。后台下载与转写共享每个模型的下载锁，避免同时写入同一个临时文件；下载支持代理环境变量和读取超时。后台准备失败会输出错误日志，任务准备失败会明确显示组件与网络原因。

在“设置 → 说话人分离模型”切换质量或快速模式；每次任务固定自己的模型选择，设置变更在后续任务中使用。模型下载或等待其他任务下载时可以取消；网络读取卡住时仍需等待读取超时，原生分离调用也只能在返回后响应取消。首次下载时间不会计入后续推理耗时估算。已有失败记录保留，可点击“重新转写”。

## 官方来源

- [ERes2NetV2 中文模型卡、训练数据、同表 EER 与 Apache-2.0 许可](https://modelscope.cn/models/iic/speech_eres2netv2_sv_zh-cn_16k-common/summary)
- [3D-Speaker 官方项目与评测](https://github.com/modelscope/3D-Speaker)
- [sherpa-onnx 说话人分离模型、Rust 可用的原生模型组合与四人样例](https://k2-fsa.github.io/sherpa/onnx/speaker-diarization/models.html)
- [原生 ONNX 声纹模型发布及校验值](https://github.com/k2-fsa/sherpa-onnx/releases/tag/speaker-recongition-models)
- [pyannote 官方 Community-1 / Precision-2 对比表（标注更新时间为 2025-09）](https://github.com/pyannote/pyannote-audio#benchmark)
