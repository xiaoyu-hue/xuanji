//! 称骨绑定：只做参数类型转换（i32 → i64）与 JSON 序列化，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 称骨：出生年月日时分，返回 JSON
#[wasm_bindgen]
pub fn bone_json(y: i32, m: i32, d: i32, hour: i32, minute: i32) -> String {
    serde_json::to_string(&xuanji_core::bone_weight(y as i64, m as i64, d as i64, hour as i64, minute as i64)).unwrap()
}
