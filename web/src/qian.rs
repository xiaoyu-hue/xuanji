//! 今日一签绑定：只做参数类型转换（i32 → i64）与 JSON 序列化，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 今日一签：同一天抽到的签固定，返回 JSON 字符串
#[wasm_bindgen]
pub fn daily_qian_json(y: i32, m: i32, d: i32) -> String {
    serde_json::to_string(&xuanji_core::daily_qian(y as i64, m as i64, d as i64)).unwrap()
}
