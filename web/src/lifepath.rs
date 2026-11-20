//! 生命灵数绑定：只做参数类型转换与 JSON 序列化，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 生命灵数：出生日期 + 目标年份，返回 JSON
#[wasm_bindgen]
pub fn lifepath_json(y: i32, m: i32, d: i32, target_year: i32) -> String {
    serde_json::to_string(&xuanji_core::lifepath(y as i64, m as i64, d as i64, target_year as i64)).unwrap()
}
