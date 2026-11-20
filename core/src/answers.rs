//! 答案之书（形式源于西方，内容为本站原创中文答案）。

const ANSWERS_JSON: &str = include_str!("../../content/answers.json");

use crate::common::hash64;

// ---------- 3. 答案之书 ----------

fn answer_bank() -> Vec<String> {
    serde_json::from_str(ANSWERS_JSON).expect("answers.json 格式错误")
}

/// 每次发问给一个新答案，随机性由调用方传入的熵（entropy）决定：
/// 命令行版用系统时间，网页版用浏览器随机数。
pub fn book_answer(entropy: u64) -> String {
    let bank = answer_bank();
    let idx = (hash64(entropy ^ 0xA17E5) % bank.len() as u64) as usize;
    bank[idx].clone()
}
