//! 今日黄历。

use lunar_rust::{
    jie_qi::JieQiRefHelper,
    lunar::LunarRefHelper,
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
