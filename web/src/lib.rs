use wasm_bindgen::prelude::*;

/// 今日黄历：入参为公历年月日，返回 JSON 字符串
#[wasm_bindgen]
pub fn almanac_json(y: i32, m: i32, d: i32) -> String {
    serde_json::to_string(&xuanji_core::almanac(y as i64, m as i64, d as i64)).unwrap()
}

/// 今日一签：同一天抽到的签固定，返回 JSON 字符串
#[wasm_bindgen]
pub fn daily_qian_json(y: i32, m: i32, d: i32) -> String {
    serde_json::to_string(&xuanji_core::daily_qian(y as i64, m as i64, d as i64)).unwrap()
}

/// 答案之书：entropy 为浏览器传入的随机数
#[wasm_bindgen]
pub fn book_answer(entropy: f64) -> String {
    xuanji_core::book_answer(entropy as u64)
}

/// 八字排盘：出生年月日时分 + 性别（1 男 0 女），返回 JSON 字符串
#[wasm_bindgen]
pub fn bazi_json(y: i32, m: i32, d: i32, hour: i32, minute: i32, gender: i32) -> String {
    serde_json::to_string(&xuanji_core::bazi(y as i64, m as i64, d as i64, hour as i64, minute as i64, gender as i64)).unwrap()
}
