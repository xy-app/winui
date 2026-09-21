# XY Runner 社区与公开插件生态 (Public Plugin Center)

欢迎来到 **XY Runner 开放社区插件中心**！

XY Runner 致力于构建安全、开放、高性能的现代化桌面自动化生态。为了确保全球用户的数据安全与系统稳定性，**XY Runner 社区与第三方插件全面基于 WebAssembly (WASI) 安全沙箱运行**。

---

## 1. 核心导航 (Navigation)

| 资源 | 说明 |
| :--- | :--- |
| 📖 **[WASM 插件开发者完全指南](./WASM_PLUGIN_GUIDE.md)** | 从零开始构建一个跨平台 WebAssembly 插件的完整实战教程，包括单动作/多动作套件、国际化 (i18n)、通信协议与市场发布流程。 |
| 🚀 **[官方示例工程 (xy_wasm_crypto)](./demo/)** | 独立的开箱即用多动作加解密与编解码套件源码（含完整 `Cargo.toml`、`manifest.json`、多语言与单元测试）。 |

---

## 2. 社区准入与安全声明 (Security Policy)

> [!NOTE]
> **关于第三方插件的运行安全：**
> 1. **100% 内存沙箱隔离**：第三方插件均在独立的 WebAssembly 虚拟机中执行，无法破坏宿主进程，无法直接发起未授权的恶意底层调用；
> 2. **一次编译，全平台通用**：同一份 `.wasm` 产物无需二次编译，可在 Windows、macOS (Intel/Apple Silicon) 与 Linux 之间无缝流转；
> 3. **极简开发门槛**：零宿主专有库依赖，仅需标准的输入输出 JSON 格式即可完成任何复杂业务逻辑。
