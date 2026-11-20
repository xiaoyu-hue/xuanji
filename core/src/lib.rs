use lunar_rust::{
    jie_qi::JieQiRefHelper,
    lunar::LunarRefHelper,
    solar::{self, SolarRefHelper},
};
use serde::Serialize;

const QIAN_JSON: &str = include_str!("../../content/qian.json");
const ANSWERS_JSON: &str = include_str!("../../content/answers.json");

/// splitmix64：把日期这类规整的种子打散成均匀的伪随机数，
/// 保证「同一天同一个人问，结果稳定；相邻两天结果跳变」。
fn hash64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D049BB133111EB);
    x ^ (x >> 31)
}

fn date_seed(y: i64, m: i64, d: i64) -> u64 {
    ((y as u64) << 16) ^ ((m as u64) << 8) ^ (d as u64)
}

// ---------- 1. 今日黄历 ----------

#[derive(Serialize)]
pub struct Almanac {
    pub solar: String,
    pub lunar_text: String,
    pub year_ganzhi: String,
    pub month_ganzhi: String,
    pub day_ganzhi: String,
    pub shengxiao: String,
    pub xingzuo: String,
    pub yi: Vec<String>,
    pub ji: Vec<String>,
    pub chong: String,
    pub sha: String,
    pub xiu: String,
    pub xiu_luck: String,
    pub pengzu_gan: String,
    pub pengzu_zhi: String,
    pub cai: String,
    pub xi: String,
    pub fu: String,
    pub tianshen: String,
    pub tianshen_luck: String,
    pub prev_jieqi: String,
    pub next_jieqi: String,
}

pub fn almanac(y: i64, m: i64, d: i64) -> Almanac {
    let solar = solar::from_ymdhms(y, m, d, 12, 0, 0);
    let lunar = solar.get_lunar();
    Almanac {
        solar: format!("{y}-{m:02}-{d:02}"),
        lunar_text: format!(
            "{}年{}月{}",
            lunar.get_year_in_chinese(),
            lunar.get_month_in_chinese(),
            lunar.get_day_in_chinese()
        ),
        year_ganzhi: lunar.get_year_in_gan_zhi(),
        month_ganzhi: lunar.get_month_in_gan_zhi(),
        day_ganzhi: lunar.get_day_in_gan_zhi(),
        shengxiao: lunar.get_year_sheng_xiao(),
        xingzuo: solar.get_xing_zuo(),
        yi: lunar.get_day_yi(Some(1)),
        ji: lunar.get_day_ji(Some(1)),
        chong: lunar.get_day_chong_desc(),
        sha: lunar.get_day_sha(),
        xiu: lunar.get_xiu(),
        xiu_luck: lunar.get_xiu_luck(),
        pengzu_gan: lunar.get_peng_zu_gan(),
        pengzu_zhi: lunar.get_peng_zu_zhi(),
        cai: lunar.get_position_cai_desc(),
        xi: lunar.get_position_xi_desc(),
        fu: lunar.get_position_fu_desc(),
        tianshen: lunar.get_day_tian_shen(),
        tianshen_luck: lunar.get_day_tian_shen_luck(),
        prev_jieqi: lunar
            .get_prev_jie_qi(Some(true))
            .map(|j| j.get_name())
            .unwrap_or_default(),
        next_jieqi: lunar
            .get_next_jie_qi(Some(true))
            .map(|j| j.get_name())
            .unwrap_or_default(),
    }
}

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

// ---------- 4. 星座运势 ----------

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
