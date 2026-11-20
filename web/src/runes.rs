//! 卢恩绑定：只做参数类型转换与 JSON 序列化，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 今日一卢恩：日期，返回 JSON
#[wasm_bindgen]
pub fn daily_rune_json(y: i32, m: i32, d: i32) -> String {
    serde_json::to_string(&xuanji_core::daily_rune(y as i64, m as i64, d as i64)).unwrap()
}

/// 三卢恩（过去/现在/未来）：熵由页面传入，返回 JSON
#[wasm_bindgen]
pub fn runes_draw3_json(entropy: f64) -> String {
    serde_json::to_string(&xuanji_core::runes_draw3(entropy as u64)).unwrap()
}
