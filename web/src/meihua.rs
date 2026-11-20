//! 梅花易数绑定：只做参数类型转换（i32 → i64）与 JSON 序列化，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 梅花时间起卦：公历年月日 + 小时，返回 JSON
#[wasm_bindgen]
pub fn meihua_json(y: i32, m: i32, d: i32, hour: i32) -> String {
    serde_json::to_string(&xuanji_core::meihua(y as i64, m as i64, d as i64, hour as i64)).unwrap()
}
