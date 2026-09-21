//! Official WebAssembly Demo Plugin: Multi-Action Crypto & Codec Suite
//!
//! Provides 3 actions:
//! 1. `CalculateHash`: Computes SHA-256, MD5, or SHA-1 hashes from input text.
//! 2. `Base64Codec`: Encodes UTF-8 strings into Base64 or decodes Base64 to text.
//! 3. `UrlCodec`: Encodes or decodes URL components.
//!
//! Designed to compile to `wasm32-wasip1` or `wasm32-unknown-unknown` without host dependencies.

use base64::prelude::*;
use md5::{Digest, Md5};
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::Sha256;
use std::collections::HashMap;

// ----------------------------------------------------------------------------
// 1. WASM Standard Communication Protocol Models
// ----------------------------------------------------------------------------

/// Standard input structure delivered by the XY Runner host
#[derive(Debug, Clone, Deserialize)]
pub struct WasmExecutionInput {
    /// Target action tag name (e.g., "CalculateHash", "Base64Codec", "UrlCodec")
    pub action_tag: String,
    /// Action configuration fields map
    #[serde(default)]
    pub data: HashMap<String, serde_json::Value>,
    /// Global and step context variables passed from workflow
    #[serde(default)]
    pub variables: HashMap<String, String>,
}

/// Standard output structure returned to the XY Runner host
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmExecutionOutput {
    /// Whether the action executed successfully
    pub success: bool,
    /// Error message if execution failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Primary step output string (often formatted JSON or raw text)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// Context variables to set or update in the workflow
    #[serde(default)]
    pub set_variables: HashMap<String, String>,
}

impl WasmExecutionOutput {
    pub fn ok(output: String, set_variables: HashMap<String, String>) -> Self {
        Self {
            success: true,
            error: None,
            output: Some(output),
            set_variables,
        }
    }

    pub fn fail(err: impl Into<String>) -> Self {
        Self {
            success: false,
            error: Some(err.into()),
            output: None,
            set_variables: HashMap::new(),
        }
    }
}

// ----------------------------------------------------------------------------
// 2. Action Logic Implementations
// ----------------------------------------------------------------------------

/// Action 1: Calculate Hash (MD5, SHA-256, SHA-1)
pub fn execute_calculate_hash(
    data: &HashMap<String, serde_json::Value>,
) -> Result<(String, HashMap<String, String>), String> {
    let input_text = extract_string_field(data, "input_text").unwrap_or_default();
    let algorithm = extract_string_field(data, "algorithm")
        .unwrap_or_else(|| "SHA256".to_string())
        .to_uppercase();
    let uppercase = extract_bool_field(data, "uppercase").unwrap_or(false);

    let bytes = input_text.as_bytes();
    let mut hash_hex = match algorithm.as_str() {
        "MD5" => {
            let mut hasher = Md5::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
        "SHA1" | "SHA-1" => {
            let mut hasher = Sha1::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
        "SHA256" | "SHA-256" | _ => {
            let mut hasher = Sha256::new();
            hasher.update(bytes);
            hex::encode(hasher.finalize())
        }
    };

    if uppercase {
        hash_hex = hash_hex.to_uppercase();
    }

    let mut set_variables = HashMap::new();
    set_variables.insert("hash_output".to_string(), hash_hex.clone());
    set_variables.insert("hash_algorithm".to_string(), algorithm.clone());

    let payload = serde_json::json!({
        "status": "success",
        "algorithm": algorithm,
        "input_length": input_text.len(),
        "hash": hash_hex
    });

    Ok((payload.to_string(), set_variables))
}

/// Action 2: Base64 Codec (Encode / Decode)
pub fn execute_base64_codec(
    data: &HashMap<String, serde_json::Value>,
) -> Result<(String, HashMap<String, String>), String> {
    let input_text = extract_string_field(data, "input_text").unwrap_or_default();
    let mode = extract_string_field(data, "mode")
        .unwrap_or_else(|| "Encode".to_string())
        .to_lowercase();

    let (result_text, bytes_count) = if mode == "decode" {
        let clean_input = input_text.trim().replace(['\r', '\n', ' '], "");
        let decoded_bytes = BASE64_STANDARD
            .decode(clean_input.as_bytes())
            .map_err(|e| format!("Invalid Base64 input string: {}", e))?;
        let len = decoded_bytes.len();
        let utf8_text = String::from_utf8(decoded_bytes)
            .map_err(|e| format!("Decoded bytes are not valid UTF-8: {}", e))?;
        (utf8_text, len)
    } else {
        let encoded = BASE64_STANDARD.encode(input_text.as_bytes());
        let len = input_text.len();
        (encoded, len)
    };

    let mut set_variables = HashMap::new();
    set_variables.insert("base64_result".to_string(), result_text.clone());

    let payload = serde_json::json!({
        "status": "success",
        "mode": if mode == "decode" { "Decode" } else { "Encode" },
        "bytes_processed": bytes_count,
        "result": result_text
    });

    Ok((payload.to_string(), set_variables))
}

/// Action 3: URL Codec (Encode / Decode)
pub fn execute_url_codec(
    data: &HashMap<String, serde_json::Value>,
) -> Result<(String, HashMap<String, String>), String> {
    let input_text = extract_string_field(data, "input_text").unwrap_or_default();
    let mode = extract_string_field(data, "mode")
        .unwrap_or_else(|| "Encode".to_string())
        .to_lowercase();

    let result_text = if mode == "decode" {
        urlencoding::decode(&input_text)
            .map(|cow| cow.into_owned())
            .map_err(|e| format!("Failed to decode URL string: {}", e))?
    } else {
        urlencoding::encode(&input_text).into_owned()
    };

    let mut set_variables = HashMap::new();
    set_variables.insert("url_result".to_string(), result_text.clone());

    let payload = serde_json::json!({
        "status": "success",
        "mode": if mode == "decode" { "Decode" } else { "Encode" },
        "result": result_text
    });

    Ok((payload.to_string(), set_variables))
}

// ----------------------------------------------------------------------------
// 3. Multi-Action Router Dispatcher
// ----------------------------------------------------------------------------

/// Dispatches execution according to the incoming `action_tag`
pub fn dispatch_action(input: WasmExecutionInput) -> WasmExecutionOutput {
    let tag = input.action_tag.as_str();
    let res = match tag {
        "CalculateHash" => execute_calculate_hash(&input.data),
        "Base64Codec" => execute_base64_codec(&input.data),
        "UrlCodec" => execute_url_codec(&input.data),
        unknown => Err(format!(
            "Unknown action tag '{}'. Available actions: CalculateHash, Base64Codec, UrlCodec",
            unknown
        )),
    };

    match res {
        Ok((out, set_vars)) => WasmExecutionOutput::ok(out, set_vars),
        Err(err) => WasmExecutionOutput::fail(err),
    }
}

// ----------------------------------------------------------------------------
// 4. Helper Extraction Functions
// ----------------------------------------------------------------------------

fn extract_string_field(data: &HashMap<String, serde_json::Value>, key: &str) -> Option<String> {
    data.get(key).and_then(|v| {
        if let Some(s) = v.as_str() {
            Some(s.to_string())
        } else if let Some(sub_val) = v.get("value") {
            sub_val.as_str().map(|s| s.to_string())
        } else {
            Some(v.to_string())
        }
    })
}

fn extract_bool_field(data: &HashMap<String, serde_json::Value>, key: &str) -> Option<bool> {
    data.get(key).and_then(|v| {
        if let Some(b) = v.as_bool() {
            Some(b)
        } else if let Some(s) = v.as_str() {
            match s.trim().to_lowercase().as_str() {
                "true" | "1" | "yes" | "t" => Some(true),
                "false" | "0" | "no" | "f" => Some(false),
                _ => None,
            }
        } else if let Some(sub_val) = v.get("value") {
            sub_val.as_bool()
        } else {
            None
        }
    })
}

// ----------------------------------------------------------------------------
// 5. C-ABI WebAssembly Exports
// ----------------------------------------------------------------------------

/// Returns a raw pointer and length to the static embedded Manifest JSON string
#[unsafe(no_mangle)]
pub extern "C" fn xy_wasm_get_manifest() -> *const u8 {
    static MANIFEST: &str = include_str!("../manifest.json");
    MANIFEST.as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn xy_wasm_get_manifest_len() -> usize {
    static MANIFEST: &str = include_str!("../manifest.json");
    MANIFEST.len()
}

/// Allocates a buffer in WASM linear memory for host-to-plugin data transmission
#[unsafe(no_mangle)]
pub extern "C" fn xy_wasm_alloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

/// Deallocates a buffer previously allocated via `xy_wasm_alloc`
#[unsafe(no_mangle)]
pub extern "C" fn xy_wasm_dealloc(ptr: *mut u8, size: usize) {
    if !ptr.is_null() && size > 0 {
        unsafe {
            let _ = Vec::from_raw_parts(ptr, 0, size);
        }
    }
}

/// Main entry point for executing an action from host
///
/// Takes a pointer to a UTF-8 JSON string matching `WasmExecutionInput`.
/// Executes the requested action and writes `WasmExecutionOutput` to an internal buffer.
#[unsafe(no_mangle)]
pub extern "C" fn xy_wasm_execute(input_ptr: *const u8, input_len: usize) -> *mut u8 {
    let input_bytes = unsafe { std::slice::from_raw_parts(input_ptr, input_len) };
    let output = match serde_json::from_slice::<WasmExecutionInput>(input_bytes) {
        Ok(parsed_input) => dispatch_action(parsed_input),
        Err(e) => WasmExecutionOutput::fail(format!("Failed to parse execution input JSON: {}", e)),
    };

    let output_json = serde_json::to_vec(&output).unwrap_or_else(|e| {
        format!(r#"{{"success":false,"error":"Serialization error: {}"}}"#, e).into_bytes()
    });

    let out_len = output_json.len();
    let mut full_payload = Vec::with_capacity(4 + out_len);
    // Prepend 4 bytes length prefix (little-endian)
    full_payload.extend_from_slice(&(out_len as u32).to_le_bytes());
    full_payload.extend_from_slice(&output_json);

    let res_ptr = full_payload.as_mut_ptr();
    std::mem::forget(full_payload);
    res_ptr
}

// ----------------------------------------------------------------------------
// 6. Comprehensive Unit Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_hash_sha256() {
        let mut data = HashMap::new();
        data.insert("input_text".to_string(), serde_json::json!("Hello World"));
        data.insert("algorithm".to_string(), serde_json::json!("SHA256"));
        data.insert("uppercase".to_string(), serde_json::json!(false));

        let (out, vars) = execute_calculate_hash(&data).expect("Hash calculation should succeed");
        assert!(out.contains("a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"));
        assert_eq!(
            vars.get("hash_output").unwrap(),
            "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"
        );
    }

    #[test]
    fn test_calculate_hash_md5_uppercase() {
        let mut data = HashMap::new();
        data.insert("input_text".to_string(), serde_json::json!("123456"));
        data.insert("algorithm".to_string(), serde_json::json!("MD5"));
        data.insert("uppercase".to_string(), serde_json::json!(true));

        let (out, vars) = execute_calculate_hash(&data).expect("MD5 calculation should succeed");
        // 123456 MD5 is e10adc3949ba59abbe56e057f20f883e
        assert!(out.contains("E10ADC3949BA59ABBE56E057F20F883E"));
        assert_eq!(
            vars.get("hash_output").unwrap(),
            "E10ADC3949BA59ABBE56E057F20F883E"
        );
    }

    #[test]
    fn test_base64_encode_and_decode_roundtrip() {
        let original_text = "Automation with XY Runner is fast & safe! 🚀";

        // Encode
        let mut encode_data = HashMap::new();
        encode_data.insert("input_text".to_string(), serde_json::json!(original_text));
        encode_data.insert("mode".to_string(), serde_json::json!("Encode"));
        let (_, encode_vars) = execute_base64_codec(&encode_data).expect("Encode should succeed");
        let encoded_str = encode_vars.get("base64_result").unwrap();

        // Decode
        let mut decode_data = HashMap::new();
        decode_data.insert("input_text".to_string(), serde_json::json!(encoded_str));
        decode_data.insert("mode".to_string(), serde_json::json!("Decode"));
        let (_, decode_vars) = execute_base64_codec(&decode_data).expect("Decode should succeed");
        let decoded_str = decode_vars.get("base64_result").unwrap();

        assert_eq!(decoded_str, original_text);
    }

    #[test]
    fn test_url_encode_and_decode_roundtrip() {
        let original_url = "https://example.com/search?keyword=自动化测试&type=all";

        // Encode
        let mut encode_data = HashMap::new();
        encode_data.insert("input_text".to_string(), serde_json::json!(original_url));
        encode_data.insert("mode".to_string(), serde_json::json!("Encode"));
        let (_, encode_vars) = execute_url_codec(&encode_data).expect("URL encode should succeed");
        let encoded_str = encode_vars.get("url_result").unwrap();
        assert!(encoded_str.contains("%E8%87%AA%E5%8A%A8%E5%8C%96"));

        // Decode
        let mut decode_data = HashMap::new();
        decode_data.insert("input_text".to_string(), serde_json::json!(encoded_str));
        decode_data.insert("mode".to_string(), serde_json::json!("Decode"));
        let (_, decode_vars) = execute_url_codec(&decode_data).expect("URL decode should succeed");
        let decoded_str = decode_vars.get("url_result").unwrap();

        assert_eq!(decoded_str, original_url);
    }

    #[test]
    fn test_multi_action_dispatcher() {
        let input = WasmExecutionInput {
            action_tag: "CalculateHash".to_string(),
            data: {
                let mut m = HashMap::new();
                m.insert("input_text".to_string(), serde_json::json!("Test"));
                m.insert("algorithm".to_string(), serde_json::json!("SHA1"));
                m.insert("uppercase".to_string(), serde_json::json!(false));
                m
            },
            variables: HashMap::new(),
        };

        let output = dispatch_action(input);
        assert!(output.success);
        assert!(output.set_variables.contains_key("hash_output"));
    }

    #[test]
    fn test_manifest_export_consistency() {
        let manifest_ptr = xy_wasm_get_manifest();
        let manifest_len = xy_wasm_get_manifest_len();
        assert!(!manifest_ptr.is_null());
        assert!(manifest_len > 0);

        let manifest_bytes = unsafe { std::slice::from_raw_parts(manifest_ptr, manifest_len) };
        let manifest_str = std::str::from_utf8(manifest_bytes).expect("Valid UTF-8 manifest");
        let json_val: serde_json::Value =
            serde_json::from_str(manifest_str).expect("Valid JSON manifest");
        assert_eq!(json_val["plugin_id"], "xy_wasm_crypto");
        assert_eq!(json_val["actions"].as_array().unwrap().len(), 3);
    }
}
