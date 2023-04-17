use regex::Regex;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use serde_wasm_bindgen::{to_value, from_value};

#[derive(Deserialize, Serialize)]
pub struct StringData {
    vector: Vec<u8>,
    offsets: Vec<usize>,
}

#[wasm_bindgen]
pub fn count_regex_matches(data: JsValue, regex: &str) -> Result<JsValue, JsValue> {
    let string_data: StringData = from_value(data).map_err(|e| e.to_string())?;

    let re = match Regex::new(regex) {
        Ok(re) => re,
        Err(e) => return Err(JsValue::from_str(&e.to_string())),
    };

    let mut matches = Vec::new();
    for i in 0..string_data.offsets.len() - 1 {
        let start = string_data.offsets[i];
        let end = string_data.offsets[i + 1];
        let substring = String::from_utf8_lossy(&string_data.vector[start..end]);
        let count = re.find_iter(&substring).count();
        matches.push(count as f32);
    }

    let result = to_value(&matches).map_err(|e| e.to_string())?;
    Ok(result)
}
