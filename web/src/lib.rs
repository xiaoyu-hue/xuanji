use wasm_bindgen::prelude::*;

/// 今日黄历，返回 JSON 字符串
#[wasm_bindgen]
pub fn almanac_json(y: i32, m: i32, d: i32) -> String {
    serde_json::to_string(&xuanji_core::almanac(y as i64, m as i64, d as i64)).unwrap_or_default()
}

/// 今日一签（同一天固定），返回 JSON 字符串
#[wasm_bindgen]
pub fn daily_qian_json(y: i32, m: i32, d: i32) -> String {
    serde_json::to_string(&xuanji_core::daily_qian(y as i64, m as i64, d as i64))
        .unwrap_or_default()
}

/// 答案之书：entropy 由网页端提供（浏览器随机数）
#[wasm_bindgen]
pub fn book_answer(entropy: f64) -> String {
    xuanji_core::book_answer(entropy as u64)
}

/// 星座下标（0-11）
#[wasm_bindgen]
pub fn zodiac_index(month: i32, day: i32) -> usize {
    xuanji_core::zodiac_index(month as i64, day as i64)
}

/// 星座名
#[wasm_bindgen]
pub fn zodiac_name(index: usize) -> String {
    xuanji_core::ZODIAC_NAMES[index % 12].to_string()
}

/// 星座运势（同星座同日固定），返回 JSON 字符串
#[wasm_bindgen]
pub fn fortune_json(sign: usize, y: i32, m: i32, d: i32) -> String {
    serde_json::to_string(&xuanji_core::zodiac_fortune(
        sign,
        y as i64,
        m as i64,
        d as i64,
    ))
    .unwrap_or_default()
}
