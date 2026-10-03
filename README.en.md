# 小友+ Public & Community Plugin Center (Public Plugin Ecosystem)

[![Discord](https://img.shields.io/discord/832082456727257118?label=Discord&logo=discord&logoColor=white&color=5865F2)](https://discord.gg/g5Fh3FtfGx)

Welcome to the **小友+ Open Community Plugin Center**!

小友+ is committed to building a safe, open, and high-performance modern desktop automation ecosystem. To guarantee user data security and system stability worldwide, **all community and third-party plugins execute strictly inside WebAssembly (WASI) sandboxes**.

---

## 1. Core Navigation

| Resource | Description |
| :--- | :--- |
| 📖 **[WebAssembly Plugin Developer Guide (CN)](./WASM_PLUGIN_GUIDE.md)** | Complete tutorial on developing cross-platform WASM plugins from scratch, covering single/multi-action suites, 8-language i18n, communication protocols, and marketplace publishing. |
| 🚀 **[Official Demo Project (xy_wasm_crypto)](./demo/)** | Turnkey reference implementation of a multi-action crypto & codec suite (with `Cargo.toml`, `manifest.json`, complete 8-language translations, and unit tests). |

---

## 2. Supported Internationalization (i18n) Locales

小友+ natively provides global multi-language localization. All public plugins are recommended to supply locale JSON files under `locales/` matching the standard 8 supported languages:

1. `zh_CN.json` - Simplified Chinese (简体中文)
2. `zh_TW.json` - Traditional Chinese (繁體中文)
3. `en.json` - English
4. `ja.json` - Japanese (日本語)
5. `ko.json` - Korean (한국어)
6. `de.json` - German (Deutsch)
7. `fr.json` - French (Français)
8. `ru.json` - Russian (Русский)

---

## 3. Security Policy & Sandbox Guarantees

> [!NOTE]
> **Third-Party Plugin Security Standards:**
> 1. **100% Memory Sandbox Isolation**: Third-party plugins execute inside isolated WebAssembly virtual machine instances without raw OS memory access or unapproved native syscalls.
> 2. **Write Once, Run Everywhere**: A single `.wasm` binary runs identically across Windows, macOS (Intel & Apple Silicon M-series), and Linux without platform-specific toolchains.
> 3. **Zero Host Lock-in**: Plugins communicate with the host via standard JSON input/output over memory buffers, keeping business logic clean and portable.
