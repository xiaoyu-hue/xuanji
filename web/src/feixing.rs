//! 九宫飞星绑定：只做参数类型转换（i32 → i64）与 JSON 序列化，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 流年九宫飞星：干支年（立春为界），返回 JSON
#[wasm_bindgen]
pub fn feixing_json(year: i32) -> String {
    serde_json::to_string(&xuanji_core::feixing(year as i64)).unwrap()
}
