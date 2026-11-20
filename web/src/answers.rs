//! 答案之书绑定：只做参数类型转换（i32 → i64）与 JSON 序列化，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 答案之书：entropy 为浏览器传入的随机数
#[wasm_bindgen]
pub fn book_answer(entropy: f64) -> String {
    xuanji_core::book_answer(entropy as u64)
}
