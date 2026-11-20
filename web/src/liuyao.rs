//! 六爻绑定：只做参数类型转换（i32 → i64）与 JSON 序列化，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 六爻起卦装卦：种子 + 起卦日期（公历），返回 JSON
#[wasm_bindgen]
pub fn liuyao_json(seed: i32, y: i32, m: i32, d: i32) -> String {
    serde_json::to_string(&xuanji_core::liuyao_cast(seed as i64, y as i64, m as i64, d as i64)).unwrap()
}
