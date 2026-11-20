//! 婚嫁择日。

use lunar_rust::{
    lunar::LunarRefHelper,
    solar::{self, SolarRefHelper},
};
use serde::Serialize;
use crate::common::{civil_from_days, days_from_civil};

// ---------- 5. 婚嫁择日 ----------

#[derive(Serialize)]
pub struct WeddingDay {
    pub solar: String,
    pub weekday: String,
    pub lunar_text: String,
    pub day_ganzhi: String,
    pub chong: String,
}

/// 婚嫁择日：在 [起, 止] 公历区间内筛「宜嫁娶」且不冲新郎、新娘生肖的日子。
/// groom / bride 为生肖单字（鼠牛虎兔龙蛇马羊猴鸡狗猪），空串表示不限。
/// 返回 JSON 数组（最多 120 条，区间最长 400 天，超限自动截断）。
pub fn wedding_days(
    y1: i64, m1: i64, d1: i64,
    y2: i64, m2: i64, d2: i64,
    groom: &str, bride: &str,
) -> String {
    let start = days_from_civil(y1, m1, d1);
    let mut end = days_from_civil(y2, m2, d2);
    if end < start {
        return "[]".to_string();
    }
    if end - start > 400 {
        end = start + 400;
    }
    const WEEK: [&str; 7] = ["日", "一", "二", "三", "四", "五", "六"];
    let mut out: Vec<WeddingDay> = Vec::new();
    let mut z = start;
    while z <= end && out.len() < 120 {
        let (y, m, d) = civil_from_days(z);
        let solar = solar::from_ymdhms(y, m, d, 12, 0, 0);
        let lunar = solar.get_lunar();
        let yi = lunar.get_day_yi(Some(1));
        let ji = lunar.get_day_ji(Some(1));
        if yi.iter().any(|x| x == "嫁娶") && !ji.iter().any(|x| x == "嫁娶") {
            let chong_sx = lunar.get_day_chong_sheng_xiao();
            if (groom.is_empty() || chong_sx != groom) && (bride.is_empty() || chong_sx != bride) {
                // 1970-01-01 是星期四，WEEK 下标 4
                let wd = WEEK[(((z + 4) % 7 + 7) % 7) as usize];
                out.push(WeddingDay {
                    solar: format!("{y}-{m:02}-{d:02}"),
                    weekday: format!("星期{wd}"),
                    lunar_text: format!(
                        "{}年{}月{}",
                        lunar.get_year_in_chinese(),
                        lunar.get_month_in_chinese(),
                        lunar.get_day_in_chinese()
                    ),
                    day_ganzhi: lunar.get_day_in_gan_zhi(),
                    chong: lunar.get_day_chong_desc(),
                });
            }
        }
        z += 1;
    }
    serde_json::to_string(&out).unwrap()
}
