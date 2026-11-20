//! 八字排盘。

use lunar_rust::{
    eight_char::{da_yun::DaYunRefHelper, yun::YunRefHelper, EightCharRefHelper},
    lunar::LunarRefHelper,
    solar::{self, SolarRefHelper},
};
use serde::Serialize;

// ---------- 4. 八字排盘 ----------

#[derive(Serialize)]
pub struct Pillar {
    pub name: String,
    pub ganzhi: String,
    pub gan: String,
    pub zhi: String,
    pub hide_gan: Vec<String>,
    pub wu_xing: String,
    pub na_yin: String,
    pub shi_shen_gan: String,
    pub shi_shen_zhi: Vec<String>,
    pub di_shi: String,
}

#[derive(Serialize)]
pub struct WxCount {
    pub name: String,
    pub count: u32,
}

#[derive(Serialize)]
pub struct DaYunItem {
    pub ganzhi: String,
    pub start_age: i64,
    pub end_age: i64,
    pub start_year: i64,
    pub end_year: i64,
}

#[derive(Serialize)]
pub struct Bazi {
    pub solar: String,
    pub lunar_text: String,
    pub pillars: Vec<Pillar>,
    pub day_gan: String,
    pub day_wuxing: String,
    pub wuxing: Vec<WxCount>,
    pub missing: Vec<String>,
    pub strongest: String,
    pub tai_yuan: String,
    pub ming_gong: String,
    pub shen_gong: String,
    pub forward: bool,
    pub start_desc: String,
    pub da_yun: Vec<DaYunItem>,
}

/// 命宫自算：lunar_rust 的 get_ming_gong 在 32 位目标（WebAssembly）上
/// 会因 usize 减法下溢而崩溃（64 位本地构建则被回绕掩盖）。
/// 公式与原库一致，改用 i64 运算：命宫支 = 26 −（月支序 + 时支序），再回绕；
/// 干支序号 = 月柱序号 −（月支序 − 命宫支序），取六十甲子。
fn ming_gong(month_ganzhi: &str, month_zhi: &str, time_zhi: &str) -> String {
    // 与原库一致的 1 起始月支表（首元为空串）：寅=1 … 子=11、丑=12
    const MONTH_ZHI: [&str; 13] = ["", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥", "子", "丑"];
    const GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
    const ZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
    let mzi = MONTH_ZHI.iter().position(|&z| z == month_zhi).unwrap_or(0) as i64;
    let tzi = MONTH_ZHI.iter().position(|&z| z == time_zhi).unwrap_or(0) as i64;
    let mut zhi_index = 26 - (mzi + tzi);
    if zhi_index > 12 {
        zhi_index -= 12;
    }
    let chars: Vec<char> = month_ganzhi.chars().collect();
    let gi = GAN.iter().position(|&g| g == chars[0].to_string().as_str()).unwrap_or(0) as i64;
    let zi = ZHI.iter().position(|&z| z == chars[1].to_string().as_str()).unwrap_or(0) as i64;
    let month_idx = (0..60).find(|&i| i % 10 == gi && i % 12 == zi).unwrap_or(0);
    let idx = ((month_idx - (mzi - zhi_index)) % 60 + 60) % 60;
    format!("{}{}", GAN[(idx % 10) as usize], ZHI[(idx % 12) as usize])
}

fn gan_wuxing(gan: &str) -> &'static str {    match gan {
        "甲" | "乙" => "木",
        "丙" | "丁" => "火",
        "戊" | "己" => "土",
        "庚" | "辛" => "金",
        _ => "水",
    }
}

/// 八字排盘。gender：1 男、0 女（影响大运顺逆与起运，与历法库约定一致）。
pub fn bazi(y: i64, m: i64, d: i64, hour: i64, minute: i64, gender: i64) -> Bazi {
    let solar = solar::from_ymdhms(y, m, d, hour, minute, 0);
    let lunar = solar.get_lunar();
    let ec = lunar.get_eight_char();
    let mk = |name: &str,
              ganzhi: String,
              gan: String,
              zhi: String,
              hide_gan: Vec<String>,
              wu_xing: String,
              na_yin: String,
              ssg: String,
              ssz: Vec<String>,
              di_shi: String|
     -> Pillar {
        Pillar {
            name: name.to_string(),
            ganzhi,
            gan,
            zhi,
            hide_gan,
            wu_xing,
            na_yin,
            shi_shen_gan: ssg,
            shi_shen_zhi: ssz,
            di_shi,
        }
    };
    let pillars = vec![
        mk("年柱", ec.get_year(), ec.get_year_gan(), ec.get_year_zhi(), ec.get_year_hide_gan(), ec.get_year_wu_xing(), ec.get_year_na_yin(), ec.get_year_shi_shen_gan(), ec.get_year_shi_shen_zhi(), ec.get_year_di_shi()),
        mk("月柱", ec.get_month(), ec.get_month_gan(), ec.get_month_zhi(), ec.get_month_hide_gan(), ec.get_month_wu_xing(), ec.get_month_na_yin(), ec.get_month_shi_shen_gan(), ec.get_month_shi_shen_zhi(), ec.get_month_di_shi()),
        mk("日柱", ec.get_day(), ec.get_day_gan(), ec.get_day_zhi(), ec.get_day_hide_gan(), ec.get_day_wu_xing(), ec.get_day_na_yin(), ec.get_day_shi_shen_gan(), ec.get_day_shi_shen_zhi(), ec.get_day_di_shi()),
        mk("时柱", ec.get_time(), ec.get_time_gan(), ec.get_time_zhi(), ec.get_time_hide_gan(), ec.get_time_wu_xing(), ec.get_time_na_yin(), ec.get_time_shi_shen_gan(), ec.get_time_shi_shen_zhi(), ec.get_time_di_shi()),
    ];
    // 五行计数：四天干 + 四地支的全部藏干，逐个计入
    let order = ["金", "木", "水", "火", "土"];
    let mut counts = [0u32; 5];
    let idx = |wx: &str| order.iter().position(|&o| o == wx).unwrap();
    for p in &pillars {
        counts[idx(gan_wuxing(&p.gan))] += 1;
        for hg in &p.hide_gan {
            counts[idx(gan_wuxing(hg))] += 1;
        }
    }
    let wuxing: Vec<WxCount> = order
        .iter()
        .enumerate()
        .map(|(i, n)| WxCount { name: n.to_string(), count: counts[i] })
        .collect();
    let missing: Vec<String> = wuxing.iter().filter(|w| w.count == 0).map(|w| w.name.clone()).collect();
    let strongest = wuxing
        .iter()
        .max_by_key(|w| w.count)
        .map(|w| w.name.clone())
        .unwrap_or_default();
    let day_gan = ec.get_day_gan();
    let tai_yuan = ec.get_tai_yuan();
    // 命宫自算（原库实现在 WebAssembly 上会崩溃，见 ming_gong 函数注释）；
    // 身宫的原库实现是 i64 安全的，继续用
    let ming_gong = ming_gong(&ec.get_month(), &lunar.get_month_zhi_exact(), &lunar.get_time_zhi());
    let shen_gong = ec.get_shen_gong();
    // 大运：get_yun 按值消费句柄，放最后调用
    let yun = ec.get_yun(gender, None);
    let forward = yun.is_forward();
    let start_desc = format!(
        "出生后 {} 年 {} 个月 {} 天起运",
        yun.get_start_year(),
        yun.get_start_month(),
        yun.get_start_day()
    );
    let da_yun: Vec<DaYunItem> = yun
        .get_da_yun(Some(10))
        .into_iter()
        .filter(|dy| !dy.get_gan_zhi().is_empty())
        .take(8)
        .map(|dy| DaYunItem {
            ganzhi: dy.get_gan_zhi(),
            start_age: dy.get_start_age(),
            // 注：lunar_rust 移植版 get_end_age() 有误（返回了结束年份），
            // 按金标准 lunar-javascript 的定义自行计算：起始年龄 + 9
            end_age: dy.get_start_age() + 9,
            start_year: dy.get_start_year(),
            end_year: dy.get_end_year(),
        })
        .collect();
    Bazi {
        solar: format!("{y}-{m:02}-{d:02} {hour:02}:{minute:02}"),
        lunar_text: format!(
            "{}年{}月{}",
            lunar.get_year_in_chinese(),
            lunar.get_month_in_chinese(),
            lunar.get_day_in_chinese()
        ),
        pillars,
        day_wuxing: gan_wuxing(&day_gan).to_string(),
        day_gan,
        wuxing,
        missing,
        strongest,
        tai_yuan,
        ming_gong,
        shen_gong,
        forward,
        start_desc,
        da_yun,
    }
}
