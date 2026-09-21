# xy_wasm_crypto: 官方 WebAssembly 插件示例 (Multi-Action Demo)

本工程是 **XY Runner** 官方提供的标准 WebAssembly 动作插件示例，演示如何构建一个具备**多动作路由分发、多语言国际化 (i18n)、安全沙箱与零系统依赖**的高性能 WASM 插件。

---

## 1. 包含动作 (Action Overview)

本插件包含 3 个高频实用的数据处理动作：

| 动作标识 (`tag`) | 中文名称 | 功能说明 |
| :--- | :--- | :--- |
| **`CalculateHash`** | 计算哈希散列 | 支持 MD5、SHA-256、SHA-1 计算，可选大写十六进制输出，结果自动写回上下文变量 `hash_output` |
| **`Base64Codec`** | Base64 编解码 | 支持 UTF-8 文本编码为 Base64，或将 Base64 字符串解码还原，结果写回 `base64_result` |
| **`UrlCodec`** | URL 编解码 | 执行 URL URI 百分号转义编码与还原，结果写回 `url_result` |

---

## 2. 目录结构 (Project Layout)

```
xy_wasm_crypto/
├── Cargo.toml         # 独立自包含的项目配置（支持 wasm32-wasip1 / wasm32-unknown-unknown）
├── manifest.json      # 插件元数据清单（定义 3 个动作、参数标签、预设项 presets）
├── README.md          # 本工程说明
├── locales/           # 多语言国际化补丁
│   ├── zh_CN.json     # 简体中文翻译
│   └── en.json        # 英文对照
└── src/
    └── lib.rs         # 核心动作实现、多动作分发路由器、WASM C-ABI 导出与单元测试
```

---

## 3. 快速上手 (Quick Start)

### 3.1 环境准备
确保已安装 Rust 工具链（推荐 Rust 1.78+）并添加 WebAssembly 编译目标：
```bash
rustup target add wasm32-wasip1
```

### 3.2 运行本地测试
在当前目录下运行标准的 Rust 单元测试：
```bash
cargo test
```
全部 6 个测试用例将自动验证：MD5/SHA256 计算、Base64 编解码往返、URL 编解码往返、多动作路由分发以及 Manifest 静态导出。

### 3.3 编译 WebAssembly 产物
执行交叉编译生成优化的 `.wasm` 文件：
```bash
cargo build --target wasm32-wasip1 --release
```
编译成功后，产物将生成在：
`target/wasm32-wasip1/release/xy_wasm_crypto.wasm`（体积仅约 200 KB）。

---

## 4. 在 XY Runner 中安装与测试

1. 创建本地用户插件目录（若不存在）：
   - **Windows**: `%USERPROFILE%\.xy-app\plugins\xy_wasm_crypto\`
   - **macOS/Linux**: `~/.xy-app/plugins/xy_wasm_crypto/`
2. 将以下文件复制到该目录下：
   - 编译出的 `xy_wasm_crypto.wasm` 重命名为 `plugin.wasm`
   - `manifest.json`
   - `locales/` 文件夹（可选，用于多语言支持）
3. 重启或打开 XY Runner 应用程序，左侧动作库的“算法”分类下将自动出现这 3 个独立的动作节点！
