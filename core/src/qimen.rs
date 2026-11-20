//! 奇门遁甲：时家奇门 · 拆补法定局 · 转盘。
//! 地盘三奇六仪按局数顺/逆布九宫数序；天盘九星随值符转至时干宫（环序恒顺时针）；
//! 人盘八门由值使按时柱旬内序数沿宫数飞步后定偏移；神盘八神随值符、阳顺阴逆。
//! 中五寄坤、天禽随天芮。与 qimen-rs（MIT）八个时刻全盘逐宫对拍一致（自检夹具）。

use lunar_rust::{
    eight_char::EightCharRefHelper,
    jie_qi::JieQiRefHelper,
    lunar::LunarRefHelper,
    solar::{self, SolarRefHelper},
};
use serde::Serialize;

const GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
const ZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
/// 三奇六仪布盘序
const YI_QI: [&str; 9] = ["戊", "己", "庚", "辛", "壬", "癸", "丁", "丙", "乙"];
/// 九星本位（宫 1..9）
const STAR_HOME: [&str; 9] = ["天蓬", "天芮", "天冲", "天辅", "天禽", "天心", "天柱", "天任", "天英"];
/// 八门本位（宫 1..9，中宫无门）
const DOOR_HOME: [&str; 9] = ["休门", "死门", "伤门", "杜门", "", "开门", "惊门", "生门", "景门"];
const GODS: [&str; 8] = ["值符", "螣蛇", "太阴", "六合", "白虎", "玄武", "九地", "九天"];
/// 顺时针环序（自坎起：北→东北→东→东南→南→西南→西→西北）
const RING: [u8; 8] = [1, 8, 3, 4, 9, 2, 7, 6];
/// 地支落宫（子坎、丑寅艮、卯震、辰巳巽、午离、未申坤、酉兑、戌亥乾）
const ZHI_PALACE: [u8; 12] = [1, 8, 8, 3, 4, 4, 9, 2, 2, 7, 6, 6];
/// 节气序（自冬至起 24）与三元局数表（上/中/下）——拆补法标准表
const TERMS: [&str; 24] = [
    "冬至", "小寒", "大寒", "立春", "雨水", "惊蛰", "春分", "清明", "谷雨", "立夏", "小满", "芒种",
    "夏至", "小暑", "大暑", "立秋", "处暑", "白露", "秋分", "寒露", "霜降", "立冬", "小雪", "大雪",
];
const JU: [[u8; 3]; 24] = [
    [1, 7, 4], [2, 8, 5], [3, 9, 6], [8, 5, 2], [9, 6, 3], [1, 7, 4],
    [3, 9, 6], [4, 1, 7], [5, 2, 8], [4, 1, 7], [5, 2, 8], [6, 3, 9],
    [9, 3, 6], [8, 2, 5], [7, 1, 4], [2, 5, 8], [1, 4, 7], [9, 3, 6],
    [7, 1, 4], [6, 9, 3], [5, 8, 2], [6, 9, 3], [5, 8, 2], [4, 7, 1],
];
/// 宫名与方位（宫 1..9）
const PALACE_NAME: [&str; 9] = ["坎", "坤", "震", "巽", "中", "乾", "兑", "艮", "离"];
const PALACE_DIR: [&str; 9] = ["北", "西南", "东", "东南", "中", "西北", "西", "东北", "南"];
/// 盘面行序（上南下北）
const DISPLAY: [u8; 9] = [4, 9, 2, 3, 5, 7, 8, 1, 6];

fn ring_index(p: u8) -> i32 {
    if p == 5 {
        return RING.iter().position(|&x| x == 2).unwrap() as i32; // 中五寄坤
    }
    RING.iter().position(|&x| x == p).unwrap() as i32
}
fn host(p: u8) -> u8 {
    if p == 5 { 2 } else { p }
}
fn rotate(p: u8, shift: i32) -> u8 {
    RING[(ring_index(p) + shift).rem_euclid(8) as usize]
}
fn gz_index(gz: &str) -> usize {
    let g = GAN.iter().position(|&x| x == gz.chars().next().unwrap().to_string()).unwrap();
    let z = ZHI.iter().position(|&x| x == gz.chars().nth(1).unwrap().to_string()).unwrap();
    (0..60).find(|&k| k % 10 == g && k % 12 == z).unwrap()
}

#[derive(Serialize)]
pub struct QmPalace {
    pub num: u8,
    pub name: String,
    pub dir: String,
    /// 地盘干
    pub earth: String,
    /// 中宫寄坤的地盘干（仅坤宫）
    pub hosted_earth: Option<String>,
    /// 天盘干（天芮宫另含中宫寄干一条，hosted=true）
    pub heaven: Vec<String>,
    pub heaven_hosted: Option<String>,
    /// 九星（天芮宫为「天芮+天禽」）
    pub star: String,
    pub door: Option<String>,
    pub god: Option<String>,
    pub void: Vec<String>,
    pub horse: bool,
}

#[derive(Serialize)]
pub struct QimenChart {
    pub jieqi: String,
    pub dun: String,
    pub ju: u8,
    pub yuan: String,
    pub futou: String,
    pub day_ganzhi: String,
    pub hour_ganzhi: String,
    pub xun: String,
    pub zhifu_star: String,
    pub zhifu_palace: u8,
    pub zhishi_door: String,
    pub zhishi_palace: u8,
    pub kong_wang: [String; 2],
    pub horse_zhi: String,
    pub horse_palace: u8,
    /// 盘面行序九宫
    pub palaces: Vec<QmPalace>,
}

/// 时家奇门排盘：公历日期 + 小时（分钟不计，节气以当日交节时刻为准的近似见开发文档）。
/// 子初换日：23 时起日柱按次日计（时家奇门惯例，与金标准 qimen-rs 的
/// zi_start 约定一致，由自检 c5 例锚定）。日柱取生效日正午的历法库
/// EightChar（避开其子时流派设定对柱的干扰），时柱由日干五鼠遁自算，
/// 节气按原始时刻取——三处口径互不干扰。
pub fn qimen_chart(y: i64, m: i64, d: i64, hour: i64) -> QimenChart {
    let (ey, em, ed) = if hour >= 23 {
        crate::common::civil_from_days(crate::common::days_from_civil(y, m, d) + 1)
    } else {
        (y, m, d)
    };
    let day_gz = solar::from_ymdhms(ey, em, ed, 12, 0, 0)
        .get_lunar()
        .get_eight_char()
        .get_day();
    // 五鼠遁：日干定子时干（甲己甲、乙庚丙、丙辛戊、丁壬庚、戊癸壬），逐时顺进
    let day_gan_idx = GAN.iter().position(|&x| x == day_gz.chars().next().unwrap().to_string()).unwrap();
    let hour_zhi_idx = ((hour + 1) / 2).rem_euclid(12) as usize;
    let hour_gan_idx = ([0usize, 2, 4, 6, 8][day_gan_idx % 5] + hour_zhi_idx) % 10;
    let hour_gz = format!("{}{}", GAN[hour_gan_idx], ZHI[hour_zhi_idx]);
    let day_idx = gz_index(&day_gz);
    let hour_idx = gz_index(&hour_gz);

    // 定局：节气（按原始时刻）× 符头元
    let jieqi = solar::from_ymdhms(y, m, d, hour, 0, 0)
        .get_lunar()
        .get_prev_jie_qi(Some(false))
        .map(|j| j.get_name())
        .unwrap_or_default();
    let term_idx = TERMS.iter().position(|&t| t == jieqi).unwrap_or(0);
    let yuan_idx = (day_idx / 5) % 3;
    let yuan = ["上元", "中元", "下元"][yuan_idx];
    let ju = JU[term_idx][yuan_idx];
    let yang = term_idx < 12;
    let sign: i32 = if yang { 1 } else { -1 };
    let futou_idx = day_idx / 5 * 5;
    let futou = format!("{}{}", GAN[futou_idx % 10], ZHI[futou_idx % 12]);

    // 旬（按时柱）：旬首仪、旬空
    let xun_idx = hour_idx / 10;
    let xun_head = format!("{}{}", GAN[(xun_idx * 10) % 10], ZHI[(xun_idx * 10) % 12]);
    let hidden = YI_QI[xun_idx];
    let kong_wang = [
        ZHI[10 - xun_idx * 2].to_string(),
        ZHI[11 - xun_idx * 2].to_string(),
    ];

    // 地盘：自局数宫起，阳顺阴逆沿宫数 1..9 布三奇六仪
    let mut earth = [""; 10];
    let mut original = 1u8; // 旬首仪落宫（值符原宫）
    let mut target = 1u8; // 时干落宫
    let hour_stem = hour_gz.chars().next().unwrap().to_string();
    let hour_stem_eff = if hour_stem == "甲" { hidden } else { hour_stem.as_str() };
    for (step, stem) in YI_QI.iter().enumerate() {
        let p = (ju as i32 - 1 + sign * step as i32).rem_euclid(9) as u8 + 1;
        earth[p as usize] = stem;
        if *stem == hidden {
            original = p;
        }
        if *stem == hour_stem_eff {
            target = p;
        }
    }

    // 天盘：值符星转至时干宫（中宫皆寄坤），九星环序恒顺时针随转
    let star_target = host(target);
    let star_shift = ring_index(star_target) - ring_index(host(original));
    // 人盘：值使门先沿宫数（含中五）按旬内序数飞步定落宫，再求环序偏移
    let door_raw = (original as i32 - 1 + sign * (hour_idx % 10) as i32).rem_euclid(9) as u8 + 1;
    let door_target = host(door_raw);
    let door_shift = ring_index(door_target) - ring_index(host(original));

    // 值符/值使取原宫本位星门；原宫为中五时按寄坤论（值符天禽、值使死门）
    let zhifu_star = STAR_HOME[(original - 1) as usize].to_string();
    let zhishi_door = if original == 5 {
        "死门".to_string()
    } else {
        DOOR_HOME[(original - 1) as usize].to_string()
    };

    // 驿马（按时支三合）
    let hour_zhi_idx = ZHI.iter().position(|&x| x == hour_gz.chars().nth(1).unwrap().to_string()).unwrap();
    let horse_zhi = ["寅", "亥", "申", "巳"][hour_zhi_idx % 4];
    let horse_palace = ZHI_PALACE[ZHI.iter().position(|&x| x == horse_zhi).unwrap()];

    // 组装九宫（先按宫号 1..9 填，再按盘面行序输出）
    let mut by_num: Vec<QmPalace> = (1..=9u8)
        .map(|n| QmPalace {
            num: n,
            name: PALACE_NAME[(n - 1) as usize].to_string(),
            dir: PALACE_DIR[(n - 1) as usize].to_string(),
            earth: earth[n as usize].to_string(),
            hosted_earth: None,
            heaven: vec![],
            heaven_hosted: None,
            star: String::new(),
            door: None,
            god: None,
            void: vec![],
            horse: n == horse_palace,
        })
        .collect();
    by_num[1].hosted_earth = Some(earth[5].to_string()); // 中五寄坤：坤宫带中宫地盘干
    for src in RING {
        let dst = rotate(src, star_shift);
        let cell = &mut by_num[(dst - 1) as usize];
        cell.star = STAR_HOME[(src - 1) as usize].to_string();
        cell.heaven = vec![earth[src as usize].to_string()];
        if src == 2 {
            // 天禽随天芮：芮宫另带天禽与中宫天盘干
            cell.star = "天芮+天禽".to_string();
            cell.heaven_hosted = Some(earth[5].to_string());
        }
        let ddst = rotate(src, door_shift);
        by_num[(ddst - 1) as usize].door = Some(DOOR_HOME[(src - 1) as usize].to_string());
    }
    // 中宫：星门神皆空，天盘无
    // 神盘：值符神居值符落宫，阳顺阴逆
    for (step, god) in GODS.iter().enumerate() {
        let p = rotate(star_target, sign * step as i32);
        by_num[(p - 1) as usize].god = Some(god.to_string());
    }
    // 空亡落宫（按时柱旬空）
    for kz in &kong_wang {
        let zi = ZHI.iter().position(|&x| x == kz).unwrap();
        by_num[(ZHI_PALACE[zi] - 1) as usize].void.push(kz.clone());
    }

    QimenChart {
        jieqi,
        dun: if yang { "阳遁".into() } else { "阴遁".into() },
        ju,
        yuan: yuan.into(),
        futou,
        day_ganzhi: day_gz,
        hour_ganzhi: hour_gz,
        xun: xun_head,
        zhifu_star,
        zhifu_palace: star_target,
        zhishi_door,
        zhishi_palace: door_target,
        kong_wang,
        horse_zhi: horse_zhi.into(),
        horse_palace,
        palaces: DISPLAY.iter().map(|&n| {
            let c = &by_num[(n - 1) as usize];
            QmPalace {
                num: c.num,
                name: c.name.clone(),
                dir: c.dir.clone(),
                earth: c.earth.clone(),
                hosted_earth: c.hosted_earth.clone(),
                heaven: c.heaven.clone(),
                heaven_hosted: c.heaven_hosted.clone(),
                star: c.star.clone(),
                door: c.door.clone(),
                god: c.god.clone(),
                void: c.void.clone(),
                horse: c.horse,
            }
        }).collect(),
    }
}
