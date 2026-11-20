//! 塔罗绑定：只做参数类型转换，不含业务逻辑（核心函数本身返回 JSON）。

use wasm_bindgen::prelude::*;

/// 今日塔罗：日期，返回 JSON
#[wasm_bindgen]
pub fn daily_tarot_json(y: i32, m: i32, d: i32) -> String {
    xuanji_core::daily_tarot(y as i64, m as i64, d as i64)
}

/// 随手抽一张：熵由页面传入
#[wasm_bindgen]
pub fn tarot_draw_json(entropy: f64) -> String {
    xuanji_core::tarot_draw(entropy as u64)
}
