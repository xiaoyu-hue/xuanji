//! 今日黄历绑定：只做参数类型转换（i32 → i64）与 JSON 序列化，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 今日黄历：入参为公历年月日，返回 JSON 字符串
#[wasm_bindgen]
pub fn almanac_json(y: i32, m: i32, d: i32) -> String {
    serde_json::to_string(&xuanji_core::almanac(y as i64, m as i64, d as i64)).unwrap()
}

/// 每日时辰吉凶：入参为公历年月日，返回十二时辰 JSON 数组
#[wasm_bindgen]
pub fn day_hours_json(y: i32, m: i32, d: i32) -> String {
    serde_json::to_string(&xuanji_core::day_hours(y as i64, m as i64, d as i64)).unwrap()
}
