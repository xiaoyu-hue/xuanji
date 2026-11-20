use wasm_bindgen::prelude::*;

/// 今日黄历：入参为公历年月日，返回 JSON 字符串
#[wasm_bindgen]
pub fn almanac_json(y: i32, m: i32, d: i32) -> String {
    serde_json::to_string(&xuanji_core::almanac(y as i64, m as i64, d as i64)).unwrap()
}

/// 今日一签：同一天抽到的签固定，返回 JSON 字符串
#[wasm_bindgen]
pub fn daily_qian_json(y: i32, m: i32, d: i32) -> String {
    serde_json::to_string(&xuanji_core::daily_qian(y as i64, m as i64, d as i64)).unwrap()
}

/// 答案之书：entropy 为浏览器传入的随机数
#[wasm_bindgen]
pub fn book_answer(entropy: f64) -> String {
    xuanji_core::book_answer(entropy as u64)
}

/// 八字排盘：出生年月日时分 + 性别（1 男 0 女），返回 JSON 字符串
#[wasm_bindgen]
pub fn bazi_json(y: i32, m: i32, d: i32, hour: i32, minute: i32, gender: i32) -> String {
    serde_json::to_string(&xuanji_core::bazi(y as i64, m as i64, d as i64, hour as i64, minute as i64, gender as i64)).unwrap()
}

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

/// 称骨：出生年月日时分，返回 JSON
#[wasm_bindgen]
pub fn bone_json(y: i32, m: i32, d: i32, hour: i32, minute: i32) -> String {
    serde_json::to_string(&xuanji_core::bone_weight(y as i64, m as i64, d as i64, hour as i64, minute as i64)).unwrap()
}

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

/// 解梦词条库（原创），返回 JSON
#[wasm_bindgen]
pub fn dreams_json() -> String {
    xuanji_core::dreams_json()
}

/// 紫微排盘：出生年月日时分 + 性别（1 男 0 女），返回 JSON
#[wasm_bindgen]
pub fn ziwei_json(y: i32, m: i32, d: i32, hour: i32, minute: i32, gender: i32) -> String {
    serde_json::to_string(&xuanji_core::ziwei_chart(
        y as i64, m as i64, d as i64, hour as i64, minute as i64, gender as i64,
    ))
    .unwrap()
}
