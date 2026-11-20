//! 霍兰德绑定：题库透传 + 计分转发，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 霍兰德题库 JSON（48 题，题项原创）
#[wasm_bindgen]
pub fn holland_json() -> String {
    xuanji_core::holland_json()
}

/// 霍兰德计分：48 字符作答串（1-5），返回 JSON
#[wasm_bindgen]
pub fn holland_score(answers: String) -> String {
    xuanji_core::holland_score(&answers)
}
