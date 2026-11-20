//! 犯太岁绑定：只做参数类型转换（i32 → i64）与 JSON 序列化，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 犯太岁：公历年份 + 本人生肖（空串只看全年名单），返回 JSON
#[wasm_bindgen]
pub fn taisui_json(year: i32, zodiac: String) -> String {
    serde_json::to_string(&xuanji_core::taisui(year as i64, &zodiac)).unwrap()
}
