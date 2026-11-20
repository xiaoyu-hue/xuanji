//! 婚嫁择日绑定：只做参数类型转换（i32 → i64）与 JSON 序列化，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 婚嫁择日：起止公历日期 + 新郎/新娘生肖（空串不限），返回 JSON 数组
#[wasm_bindgen]
pub fn wedding_json(
    y1: i32, m1: i32, d1: i32,
    y2: i32, m2: i32, d2: i32,
    groom: String, bride: String,
) -> String {
    xuanji_core::wedding_days(
        y1 as i64, m1 as i64, d1 as i64,
        y2 as i64, m2 as i64, d2 as i64,
        &groom, &bride,
    )
}
