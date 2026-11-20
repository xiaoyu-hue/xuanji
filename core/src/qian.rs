//! 今日一签：按日期定签，同一天结果固定。

const QIAN_JSON: &str = include_str!("../../content/qian.json");

use crate::common::{date_seed, hash64};
use serde::Serialize;


// ---------- 2. 今日一签 ----------

#[derive(Serialize, serde::Deserialize, Clone)]
pub struct Qian {
    pub id: u32,
    pub grade: String,
    pub poem: Vec<String>,
    pub jie: String,
}

fn qian_bank() -> Vec<Qian> {
    serde_json::from_str(QIAN_JSON).expect("qian.json 格式错误")
}

/// 同一天抽到的签固定不变（日期做种子），过一天换一签。
pub fn daily_qian(y: i64, m: i64, d: i64) -> Qian {
    let bank = qian_bank();
    let idx = (hash64(date_seed(y, m, d)) % bank.len() as u64) as usize;
    bank[idx].clone()
}
