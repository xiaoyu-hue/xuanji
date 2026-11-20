//! 16 型人格绑定：题库透传 + 计分转发，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 人格题库 JSON（题目 + 16 型文案）
#[wasm_bindgen]
pub fn personality_json() -> String {
    xuanji_core::personality_json()
}

/// 人格计分：48 字符作答串（a/b），返回 JSON
#[wasm_bindgen]
pub fn personality_score(answers: String) -> String {
    xuanji_core::personality_score(&answers)
}
