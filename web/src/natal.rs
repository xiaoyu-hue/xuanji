//! 西洋星盘绑定：只做参数类型转换与 JSON 序列化，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 本命盘：公历日期 + 北京时间（时，分）+ 出生地经纬度，返回 JSON
#[wasm_bindgen]
pub fn natal_json(y: i32, m: i32, d: i32, hh: i32, mm: i32, lon_e: f64, lat: f64) -> String {
    serde_json::to_string(&xuanji_core::natal_chart(
        y as i64, m as i64, d as i64, hh as i64, mm as i64, lon_e, lat,
    ))
    .unwrap()
}
