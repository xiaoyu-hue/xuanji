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

/// 择日扫描内核：筛「宜含 activity 且忌不含 activity」且不冲两个生肖的日子。
/// avoid1 / avoid2 为生肖单字（鼠牛虎兔龙蛇马羊猴鸡狗猪），空串表示不限。
/// 返回 JSON 数组（最多 120 条，区间最长 400 天，超限自动截断）。
fn scan_days(
    y1: i64, m1: i64, d1: i64,
    y2: i64, m2: i64, d2: i64,
    activity: &str, avoid1: &str, avoid2: &str,
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
        if yi.iter().any(|x| x == activity) && !ji.iter().any(|x| x == activity) {
            let chong_sx = lunar.get_day_chong_sheng_xiao();
            if (avoid1.is_empty() || chong_sx != avoid1) && (avoid2.is_empty() || chong_sx != avoid2) {
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

/// 婚嫁择日：`scan_days` 的嫁娶特例（保留原签名，金标准用例不变）。
pub fn wedding_days(
    y1: i64, m1: i64, d1: i64,
    y2: i64, m2: i64, d2: i64,
    groom: &str, bride: &str,
) -> String {
    scan_days(y1, m1, d1, y2, m2, d2, "嫁娶", groom, bride)
}

/// 万事择日：按任意事项（如 嫁娶 / 移徙 / 开市 / 出行）筛吉日，规则同婚嫁择日。
pub fn day_search(
    y1: i64, m1: i64, d1: i64,
    y2: i64, m2: i64, d2: i64,
    activity: &str, avoid1: &str, avoid2: &str,
) -> String {
    scan_days(y1, m1, d1, y2, m2, d2, activity, avoid1, avoid2)
}
