//! 八字排盘绑定：只做参数类型转换（i32 → i64）与 JSON 序列化，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 八字排盘：出生年月日时分 + 性别（1 男 0 女），返回 JSON 字符串
#[wasm_bindgen]
pub fn bazi_json(y: i32, m: i32, d: i32, hour: i32, minute: i32, gender: i32) -> String {
    serde_json::to_string(&xuanji_core::bazi(y as i64, m as i64, d as i64, hour as i64, minute as i64, gender as i64)).unwrap()
}
