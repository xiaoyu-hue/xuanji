//! 大五人格绑定：题库透传 + 计分转发，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 大五题库 JSON（IPIP 50 题，公有领域条目 + 本站翻译）
#[wasm_bindgen]
pub fn bigfive_json() -> String {
    xuanji_core::bigfive_json()
}

/// 大五计分：50 字符作答串（1-5），返回 JSON
#[wasm_bindgen]
pub fn bigfive_score(answers: String) -> String {
    xuanji_core::bigfive_score(&answers)
}
