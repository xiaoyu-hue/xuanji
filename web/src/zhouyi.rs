//! 周易抽卦绑定：只做参数类型转换（i32 → i64）与 JSON 序列化，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 周易抽卦：种子（页面每次点击传入新随机数），返回 JSON
#[wasm_bindgen]
pub fn zhouyi_draw_json(seed: i32) -> String {
    serde_json::to_string(&xuanji_core::zhouyi_draw(seed as i64)).unwrap()
}
