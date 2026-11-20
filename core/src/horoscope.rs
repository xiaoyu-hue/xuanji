//! 星座运势（西方馆）：按公历月日定星座，每日运势以日期+星座为种子确定。
//! 2.1.0~2.3.x 曾上线、3.0.0 下线，5.5.0 按新架构恢复（逻辑与文案沿用原版）。

use serde::Serialize;
use crate::common::{date_seed, hash64};

pub const ZODIAC_NAMES: [&str; 12] = [
    "白羊座", "金牛座", "双子座", "巨蟹座", "狮子座", "处女座", "天秤座", "天蝎座",
    "射手座", "摩羯座", "水瓶座", "双鱼座",
];

/// 按公历月日查星座，返回 0-11 下标。
pub fn zodiac_index(month: i64, day: i64) -> usize {
    // 每月「新星座」的起始日：如 1 月 20 日起是水瓶座，此前仍是摩羯座
    let bounds = [20, 19, 21, 20, 21, 22, 23, 23, 23, 24, 23, 22];
    let m = month as u64;
    if (day as u64) < bounds[(m - 1) as usize] {
        ((m + 8) % 12) as usize
    } else {
        ((m + 9) % 12) as usize
    }
}

const LUCKY_COLORS: [&str; 12] = [
    "石榴红", "杏子黄", "薄荷绿", "月光白", "宝蓝", "豆沙粉", "墨黑", "葡萄紫",
    "橘子橙", "青花蓝", "银灰", "樱花粉",
];

const GUIDE_HIGH: [&str; 6] = [
    "宜主动出击，想做的事今天就开口。",
    "贵人运旺，重要的话挑今天说。",
    "适合开启新计划，第一步迈出去就顺了。",
    "财运在线，但见好就收，别贪心。",
    "表达欲和感染力都强，适合展示自己。",
    "拖延很久的事，今天能一口气推进大半。",
];
const GUIDE_MID: [&str; 6] = [
    "宜稳扎稳打，把手头的事收尾。",
    "适合整理和复盘，旧物里有新发现。",
    "与人相处多听少说，误会自然消散。",
    "小步前进也是前进，别跟人比速度。",
    "适合陪家人朋友，关系运在升温。",
    "钱财宜守不宜攻，冲动消费先放一夜。",
];
const GUIDE_LOW: [&str; 6] = [
    "宜躺平蓄力，今天不适合硬碰硬。",
    "少做决定，多睡觉，醒来再说。",
    "出门注意细节，钥匙钱包多检查一遍。",
    "情绪容易上头，重要的话明天再说。",
    "适合独处，给自己留一段空白时间。",
    "旧问题可能翻出来，平常心应对就好。",
];

#[derive(Serialize)]
pub struct Fortune {
    pub sign: String,
    pub overall: u32,
    pub love: u32,
    pub career: u32,
    pub wealth: u32,
    pub lucky_color: String,
    pub lucky_number: u32,
    pub guide: String,
}

/// 同一个星座在同一天的运势固定（日期+星座做种子）。
pub fn zodiac_fortune(sign: usize, y: i64, m: i64, d: i64) -> Fortune {
    let base = hash64(date_seed(y, m, d) ^ ((sign as u64) << 40));
    let score = |salt: u64| -> u32 { 55 + (hash64(base ^ salt) % 45) as u32 };
    let overall = score(1);
    let bank: &[&str] = if overall >= 85 {
        &GUIDE_HIGH
    } else if overall >= 70 {
        &GUIDE_MID
    } else {
        &GUIDE_LOW
    };
    Fortune {
        sign: ZODIAC_NAMES[sign].to_string(),
        overall,
        love: score(2),
        career: score(3),
        wealth: score(4),
        lucky_color: LUCKY_COLORS[(hash64(base ^ 5) % 12) as usize].to_string(),
        lucky_number: 1 + (hash64(base ^ 6) % 9) as u32,
        guide: bank[(hash64(base ^ 7) % bank.len() as u64) as usize].to_string(),
    }
}
