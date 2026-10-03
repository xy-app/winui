# 小友+ 社区与公开插件生态 (Public Plugin Center)

[![Discord](https://img.shields.io/discord/832082456727257118?label=Discord&logo=discord&logoColor=white&color=5865F2)](https://discord.gg/g5Fh3FtfGx)

欢迎来到 **小友+ 开放社区插件中心**！

小友+ 致力于构建安全、开放、高性能的现代化桌面自动化生态。为了确保全球用户的数据安全与系统稳定性，**小友+ 社区与第三方插件全面基于 WebAssembly (WASI) 安全沙箱运行**。

---

## 1. 核心导航 (Navigation)

| 资源 | 说明 |
| :--- | :--- |
| 📖 **[WASM 插件开发者完全指南](./WASM_PLUGIN_GUIDE.md)** | 从零开始构建一个跨平台 WebAssembly 插件的完整实战教程，包括单动作/多动作套件、国际化 (i18n)、通信协议与市场发布流程。 |
| 🚀 **[官方示例工程 (xy_wasm_crypto)](./demo/)** | 独立的开箱即用多动作加解密与编解码套件源码（含完整 `Cargo.toml`、`manifest.json`、全套 8 种多语言与单元测试）。 |
| 🌐 **[English Documentation (README.en.md)](./README.en.md)** | English documentation for public and community WebAssembly plugin developers. |

---

## 2. 官方标准 8 种国际化多语言规范 (Standard 8 Locales)

小友+ 具有全平台原生多语言架构。所有公开及社区插件推荐在 `locales/` 目录下提供以下标准 8 种语言的 JSON 补丁文件：

1. `zh_CN.json` - 简体中文 (Simplified Chinese)
2. `zh_TW.json` - 繁体中文 (Traditional Chinese)
3. `en.json` - 英语 (English)
4. `ja.json` - 日语 (日本語)
5. `ko.json` - 韩语 (한국어)
6. `de.json` - 德语 (Deutsch)
7. `fr.json` - 法语 (Français)
8. `ru.json` - 俄语 (Русский)

---

## 3. 社区准入与安全声明 (Security Policy)

> [!NOTE]
> **关于第三方插件的运行安全：**
> 1. **100% 内存沙箱隔离**：第三方插件均在独立的 WebAssembly 虚拟机中执行，无法破坏宿主进程，无法直接发起未授权的恶意底层调用；
> 2. **一次编译，全平台通用**：同一份 `.wasm` 产物无需二次编译，可在 Windows、macOS (Intel/Apple Silicon) 与 Linux 之间无缝流转；
> 3. **极简开发门槛**：零宿主专有库依赖，仅需标准的输入输出 JSON 格式即可完成任何复杂业务逻辑。
