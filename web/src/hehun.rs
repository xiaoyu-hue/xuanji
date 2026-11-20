//! 合婚绑定：只做参数类型转换（i32 → i64）与 JSON 序列化，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 合婚：两人出生年月日时分，返回 JSON
#[wasm_bindgen]
pub fn hehun_json(
    ay: i32, am: i32, ad: i32, ah: i32, amin: i32,
    by: i32, bm: i32, bd: i32, bh: i32, bmin: i32,
) -> String {
    serde_json::to_string(&xuanji_core::hehun(
        ay as i64, am as i64, ad as i64, ah as i64, amin as i64,
        by as i64, bm as i64, bd as i64, bh as i64, bmin as i64,
    ))
    .unwrap()
}
