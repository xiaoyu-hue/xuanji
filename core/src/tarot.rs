//! 今日塔罗（西方馆）：22 张大阿卡纳，每日一抽固定、随手抽凭熵。
//! 2.2.0 曾上线、3.0.0 下线，5.5.0 按新架构恢复（牌库与牌义沿用原版原创）。

use crate::common::{date_seed, hash64};

/// 22 张大阿卡纳牌库（文件：content/tarot.json，构建时嵌入；牌义为原创）
pub const TAROT_JSON: &str = include_str!("../../content/tarot.json");

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct TarotCard {
    pub num: u32,
    pub name: String,
    pub en: String,
    pub kw: String,
    pub up: String,
    pub rev: String,
}

fn tarot_bank() -> Vec<TarotCard> {
    serde_json::from_str(TAROT_JSON).expect("tarot.json 格式错误")
}

fn tarot_pick(h: u64) -> String {
    let bank = tarot_bank();
    let x = hash64(h);
    let c = &bank[(x % bank.len() as u64) as usize];
    let reversed = (x >> 8) % 2 == 1;
    serde_json::json!({
        "num": c.num, "name": c.name, "en": c.en, "kw": c.kw,
        "reversed": reversed,
        "meaning": if reversed { &c.rev } else { &c.up }
    })
    .to_string()
}

/// 今日塔罗：同一天抽到的牌（含正/逆位）固定不变
pub fn daily_tarot(y: i64, m: i64, d: i64) -> String {
    tarot_pick(date_seed(y, m, d) ^ 0x7A207)
}

/// 随手抽一张：随机性由调用方传入的熵决定
pub fn tarot_draw(entropy: u64) -> String {
    tarot_pick(entropy ^ 0x7A208)
}
