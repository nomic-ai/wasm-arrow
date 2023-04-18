use js_sys::{Float32Array};
use regex::RegexBuilder;
use std::str;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn count_regex_matches(vector: &[u8], offsets: &[usize], regex: &str, case_insensitive: bool) -> Float32Array {
    let regex = match RegexBuilder::new(regex)
        .case_insensitive(case_insensitive)
        .build() {
            Ok(r) => r,
            Err(_) => {
                let mut array = Float32Array::new_with_length(offsets.len() as u32 - 1);
                for i in 0..(offsets.len() - 1) {
                    array.set_index(i as u32, -1.0);
                }
                return array;
            }
    };
    
    let mut matches = Vec::with_capacity(offsets.len() - 1);
    for i in 0..(offsets.len() - 1) {
        let start = offsets[i];
        let end = offsets[i + 1];
        let string_slice = &vector[start..end];
        let count = match str::from_utf8(string_slice) {
            Ok(valid_str) => regex.find_iter(valid_str).count() as f32,
            Err(_) => -1.0,
        };
        matches.push(count);
    }

    let array = Float32Array::new_with_length(matches.len() as u32);
    array.copy_from(&matches);
    array
}
