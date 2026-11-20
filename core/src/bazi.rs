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
    /// 本柱所带神煞（传统查表法，见 shensha_for 注）
    pub shensha: Vec<String>,
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

/// 神煞查表（传统通行表，已与多个公开来源交叉核对；个别口诀异文见开发文档）：
/// - 以日干起（查各柱地支）：天乙贵人、太极贵人、文昌贵人、国印贵人、禄神；
/// - 以年支 / 日支的三合局起（查各柱地支）：桃花、驿马、华盖、将星；
/// - 以月支起：天德贵人（目标或干或支）、月德贵人（查各柱天干）；
/// - 以年支起（查各柱地支）：红鸾、天喜、孤辰、寡宿。
/// 羊刃各派口径分歧较大，本版不收。
fn shensha_for(gan: &str, zhi: &str, day_gan: &str, year_zhi: &str, day_zhi: &str, month_zhi: &str) -> Vec<String> {
    const GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
    const ZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
    let gi = |g: &str| GAN.iter().position(|&x| x == g).unwrap_or(0);
    let zi = |z: &str| ZHI.iter().position(|&x| x == z).unwrap_or(0);
    let dg = gi(day_gan);
    // 日干 → 目标地支组（下标为日干序）
    const TIANYI: [&[usize]; 10] = [&[1, 7], &[0, 8], &[11, 9], &[11, 9], &[1, 7], &[0, 8], &[1, 7], &[6, 2], &[3, 5], &[3, 5]];
    const TAIJI: [&[usize]; 10] = [&[0, 6], &[0, 6], &[3, 9], &[3, 9], &[4, 10, 1, 7], &[4, 10, 1, 7], &[2, 11], &[2, 11], &[5, 8], &[5, 8]];
    const WENCHANG: [usize; 10] = [5, 6, 8, 9, 8, 9, 11, 0, 2, 3];
    const GUOYIN: [usize; 10] = [10, 11, 1, 2, 1, 2, 5, 6, 8, 9];
    const LUSHEN: [usize; 10] = [2, 3, 5, 6, 5, 6, 8, 9, 11, 0];
    // 三合局：0 水（申子辰）1 火（寅午戌）2 金（巳酉丑）3 木（亥卯未），按地支序查
    const GROUP: [usize; 12] = [0, 2, 1, 3, 0, 2, 1, 3, 0, 2, 1, 3];
    const TAOHUA: [usize; 4] = [9, 3, 6, 0];
    const YIMA: [usize; 4] = [2, 8, 11, 5];
    const HUAGAI: [usize; 4] = [4, 10, 1, 7];
    const JIANGXING: [usize; 4] = [0, 6, 9, 3];
    // 天德：按月支序（子..亥），目标 (是干?, 序号)：子巳、丑庚、寅丁、卯申、
    // 辰壬、巳辛、午亥、未甲、申癸、酉寅、戌丙、亥乙
    const TIANDE: [(bool, usize); 12] = [
        (false, 5), (true, 6), (true, 3), (false, 8), (true, 8), (true, 7),
        (false, 11), (true, 0), (true, 9), (false, 2), (true, 2), (true, 1),
    ];
    // 月德：按月支三合局序 [水,火,金,木] → 天干序
    const YUEDE: [usize; 4] = [8, 2, 6, 0];
    // 红鸾 / 天喜：按年支序的目标地支序
    const HONGLUAN: [usize; 12] = [3, 2, 1, 0, 11, 10, 9, 8, 7, 6, 5, 4];
    // 孤辰 / 寡宿：按年支三会组 [北亥子丑, 东寅卯辰, 南巳午未, 西申酉戌]
    const HUI_GROUP: [usize; 12] = [0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 0];
    const GUCHEN: [usize; 4] = [2, 5, 8, 11];
    const GUASU: [usize; 4] = [10, 1, 4, 7];

    let pzi = zi(zhi);
    let pgi = gi(gan);
    let mut out: Vec<String> = Vec::new();
    let mut push = |hit: bool, name: &str| {
        if hit {
            out.push(name.to_string());
        }
    };
    push(TIANYI[dg].contains(&pzi), "天乙贵人");
    push(TAIJI[dg].contains(&pzi), "太极贵人");
    push(WENCHANG[dg] == pzi, "文昌贵人");
    push(GUOYIN[dg] == pzi, "国印贵人");
    let (td_is_gan, td_idx) = TIANDE[zi(month_zhi)];
    push(if td_is_gan { pgi == td_idx } else { pzi == td_idx }, "天德贵人");
    push(YUEDE[GROUP[zi(month_zhi)]] == pgi, "月德贵人");
    push(LUSHEN[dg] == pzi, "禄神");
    let yz = zi(year_zhi);
    push(HONGLUAN[yz] == pzi, "红鸾");
    push((HONGLUAN[yz] + 6) % 12 == pzi, "天喜");
    let groups = [GROUP[yz], GROUP[zi(day_zhi)]];
    push(groups.iter().any(|&g| JIANGXING[g] == pzi), "将星");
    push(groups.iter().any(|&g| TAOHUA[g] == pzi), "桃花");
    push(groups.iter().any(|&g| YIMA[g] == pzi), "驿马");
    push(groups.iter().any(|&g| HUAGAI[g] == pzi), "华盖");
    push(GUCHEN[HUI_GROUP[yz]] == pzi, "孤辰");
    push(GUASU[HUI_GROUP[yz]] == pzi, "寡宿");
    out
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
            shensha: Vec::new(),
        }
    };
    let mut pillars = vec![
        mk("年柱", ec.get_year(), ec.get_year_gan(), ec.get_year_zhi(), ec.get_year_hide_gan(), ec.get_year_wu_xing(), ec.get_year_na_yin(), ec.get_year_shi_shen_gan(), ec.get_year_shi_shen_zhi(), ec.get_year_di_shi()),
        mk("月柱", ec.get_month(), ec.get_month_gan(), ec.get_month_zhi(), ec.get_month_hide_gan(), ec.get_month_wu_xing(), ec.get_month_na_yin(), ec.get_month_shi_shen_gan(), ec.get_month_shi_shen_zhi(), ec.get_month_di_shi()),
        mk("日柱", ec.get_day(), ec.get_day_gan(), ec.get_day_zhi(), ec.get_day_hide_gan(), ec.get_day_wu_xing(), ec.get_day_na_yin(), ec.get_day_shi_shen_gan(), ec.get_day_shi_shen_zhi(), ec.get_day_di_shi()),
        mk("时柱", ec.get_time(), ec.get_time_gan(), ec.get_time_zhi(), ec.get_time_hide_gan(), ec.get_time_wu_xing(), ec.get_time_na_yin(), ec.get_time_shi_shen_gan(), ec.get_time_shi_shen_zhi(), ec.get_time_di_shi()),
    ];
    // 神煞：以日干、年支、日支、月支为基准逐柱查表（表见 shensha_for 注）
    {
        let (yg, yz, dz, mz) = (
            pillars[2].gan.clone(),
            pillars[0].zhi.clone(),
            pillars[2].zhi.clone(),
            pillars[1].zhi.clone(),
        );
        for p in &mut pillars {
            p.shensha = shensha_for(&p.gan, &p.zhi, &yg, &yz, &dz, &mz);
        }
    }
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
