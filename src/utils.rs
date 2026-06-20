use crate::types::{Color, Key};
use std::collections::HashMap;

pub fn parse_hex(s: &str) -> u32 {
    let s = s.trim_start_matches("0x");
    u32::from_str_radix(s, 16).unwrap_or(0)
}

pub fn build_key_lookup(keys: &[Key]) -> HashMap<String, String> {
    keys.iter()
        .map(|key| (key.value.clone(), key.name.clone()))
        .collect()
}

pub fn extract_color_from_lights(lights: &[u8], pos: usize) -> Color {
    if pos >= 126 || pos + 252 >= lights.len() {
        return Color::default();
    }

    Color::create(lights[pos], lights[pos + 126], lights[pos + 252])
}
