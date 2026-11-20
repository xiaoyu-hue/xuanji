//! 星座运势绑定：只做参数类型转换与 JSON 序列化，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 星座下标（0-11）：按公历月日
#[wasm_bindgen]
pub fn zodiac_index(month: i32, day: i32) -> usize {
    xuanji_core::zodiac_index(month as i64, day as i64)
}

/// 星座名
#[wasm_bindgen]
pub fn zodiac_name(index: usize) -> String {
    xuanji_core::ZODIAC_NAMES[index % 12].to_string()
}

/// 今日运势：星座下标 + 日期，返回 JSON
#[wasm_bindgen]
pub fn zodiac_fortune_json(sign: usize, y: i32, m: i32, d: i32) -> String {
    serde_json::to_string(&xuanji_core::zodiac_fortune(sign % 12, y as i64, m as i64, d as i64)).unwrap()
}
