//! 今日黄历。

use lunar_rust::{
    jie_qi::JieQiRefHelper,
    lunar::LunarRefHelper,
    lunar_time::{LunarTime, LunarTimeRefHelper},
    solar::{self, SolarRefHelper},
};
use serde::Serialize;

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

// ---------- 每日时辰吉凶 ----------

#[derive(Serialize)]
pub struct HourLuck {
    pub zhi: String,
    pub ganzhi: String,
    /// 时钟区间（如 23:00–00:59）
    pub range: String,
    /// 值时天神（如 青龙 / 白虎）
    pub tianshen: String,
    /// 天神吉凶：吉 / 凶
    pub luck: String,
}

/// 十二天神（黄道六神为吉、黑道六神为凶），顺序同金标准 TIAN_SHEN 表（去掉占位空串）。
const TIAN_SHEN: [&str; 12] = [
    "青龙", "明堂", "天刑", "朱雀", "金匮", "天德", "白虎", "玉堂", "天牢", "玄武", "司命", "勾陈",
];
/// 日支 → 天神起始偏移（金标准 ZHI_TIAN_SHEN_OFFSET，按子丑寅卯…亥序）。
const ZHI_TIAN_SHEN_OFFSET: [usize; 12] = [4, 2, 0, 10, 8, 6, 4, 2, 0, 10, 8, 6];
const ZHI12: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

/// 值时天神与吉凶（自算）：地基 lunar_rust 的 `get_tian_shen` 移植时漏了
/// 金标准公式里的 `% 12`，下标越界直接崩溃（5.1.0 实测，与八字命宫同属
/// 地基移植缺陷），故按金标准公式在此自算：
/// 天神 = TIAN_SHEN[(时支序 + 日支偏移) % 12]；黄道六神为吉、黑道六神为凶。
fn hour_tian_shen(day_zhi: &str, hour_zhi: &str) -> (String, String) {
    let di = ZHI12.iter().position(|&z| z == day_zhi).unwrap_or(0);
    let hi = ZHI12.iter().position(|&z| z == hour_zhi).unwrap_or(0);
    let name = TIAN_SHEN[(hi + ZHI_TIAN_SHEN_OFFSET[di]) % 12];
    let luck = if ["青龙", "明堂", "金匮", "天德", "玉堂", "司命"].contains(&name) {
        "吉"
    } else {
        "凶"
    };
    (name.to_string(), luck.to_string())
}

/// 每日十二时辰吉凶：以当天农历日期 + 每时辰的代表整点构造时辰取时干支，
/// 天神吉凶按金标准公式自算（见 hour_tian_shen 注）。已与金标准
/// lunar-javascript 逐项对拍。子时按早子时论（属当日），区间写作 23:00–00:59。
pub fn day_hours(y: i64, m: i64, d: i64) -> Vec<HourLuck> {
    // (代表小时, 时钟区间)；地支由构造出的时辰对象返回，不在此写死
    const SLOTS: [(i64, &str); 12] = [
        (0, "23:00–00:59"),
        (2, "01:00–02:59"),
        (4, "03:00–04:59"),
        (6, "05:00–06:59"),
        (8, "07:00–08:59"),
        (10, "09:00–10:59"),
        (12, "11:00–12:59"),
        (14, "13:00–14:59"),
        (16, "15:00–16:59"),
        (18, "17:00–18:59"),
        (20, "19:00–20:59"),
        (22, "21:00–22:59"),
    ];
    let solar = solar::from_ymdhms(y, m, d, 12, 0, 0);
    let lunar = solar.get_lunar();
    let (ly, lm, ld) = (lunar.get_year(), lunar.get_month(), lunar.get_day());
    let day_zhi = lunar.get_day_zhi_exact();
    SLOTS
        .iter()
        .map(|&(h, range)| {
            let t = LunarTime::from_ymd_hms(ly, lm, ld, h, 0, 0);
            let zhi = t.get_zhi();
            let (tianshen, luck) = hour_tian_shen(&day_zhi, &zhi);
            HourLuck {
                zhi,
                ganzhi: t.get_gan_zhi(),
                range: range.to_string(),
                tianshen,
                luck,
            }
        })
        .collect()
}
