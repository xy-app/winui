# xy_wasm_crypto: Official WebAssembly Plugin Demo (Multi-Action Suite)

[![Discord](https://img.shields.io/discord/832082456727257118?label=Discord&logo=discord&logoColor=white&color=5865F2)](https://discord.gg/g5Fh3FtfGx)

This project is the official reference WebAssembly action plugin provided by **小友+**, demonstrating how to build a high-performance WASM plugin with **multi-action routing, full 8-language internationalization (i18n), memory sandbox isolation, and zero host dependencies**.

---

## 1. Action Overview

This plugin contains 3 high-frequency data processing actions:

| Action Tag | Display Name | Description |
| :--- | :--- | :--- |
| **`CalculateHash`** | Calculate Hash | Computes MD5, SHA-256, or SHA-1 hashes with optional uppercase hexadecimal output; writes result back to context variable `hash_output` |
| **`Base64Codec`** | Base64 Codec | Converts UTF-8 text to Base64 or decodes Base64 back to text; writes result to `base64_result` |
| **`UrlCodec`** | URL Codec | Encodes or decodes URL URI percent-encoded components; writes result to `url_result` |

---

## 2. Project Layout

```
xy_wasm_crypto/
├── Cargo.toml         # Self-contained project configuration (supports wasm32-wasip1 / wasm32-unknown-unknown)
├── manifest.json      # Plugin metadata manifest (defines 3 actions, fields, presets)
├── README.md          # Chinese project documentation
├── README.en.md       # English project documentation
├── locales/           # Full 8-language official standard i18n patch suite
│   ├── zh_CN.json     # Simplified Chinese (简体中文)
│   ├── zh_TW.json     # Traditional Chinese (繁體中文)
│   ├── en.json        # English
│   ├── ja.json        # Japanese (日本語)
│   ├── ko.json        # Korean (한국어)
│   ├── de.json        # German (Deutsch)
│   ├── fr.json        # French (Français)
│   └── ru.json        # Russian (Русский)
└── src/
    └── lib.rs         # Action implementations, router, WASM C-ABI exports and unit tests
```

---

## 3. Quick Start

### 3.1 Prerequisites
Ensure the Rust toolchain is installed (Rust 1.78+ recommended) and add the WebAssembly target:
```bash
rustup target add wasm32-wasip1
```

### 3.2 Run Unit Tests
Run standard Rust unit tests inside this directory:
```bash
cargo test
```
All 7 unit tests will verify: MD5/SHA-256 calculation, Base64 encode/decode roundtrip, URL codec roundtrip, multi-action routing dispatcher, Manifest export consistency, and 8-language i18n locale integrity.

### 3.3 Compile WebAssembly Binary
Execute cross-compilation to build the optimized `.wasm` component:
```bash
cargo build --target wasm32-wasip1 --release
```
Upon completion, the binary artifact is located at:
`target/wasm32-wasip1/release/xy_wasm_crypto.wasm` (~200 KB).

---

## 4. Install & Test in 小友+

1. Create local user plugin directory (if not exists):
   - **Windows**: `%USERPROFILE%\.xy-app\plugins\xy_wasm_crypto\`
   - **macOS / Linux**: `~/.xy-app/plugins/xy_wasm_crypto/`
2. Copy the following files into the folder:
   - Rename `xy_wasm_crypto.wasm` to `plugin.wasm`
   - `manifest.json`
   - `locales/` folder (enables full 8-language localization)
3. Launch or restart 小友+; all 3 actions will automatically appear under the **Algorithm** category on the action panel!
