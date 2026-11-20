//! 紫微斗数排盘（安星口径对照开源库 iztro 验算）。

use lunar_rust::{
    lunar::LunarRefHelper,
    solar::{self, SolarRefHelper},
};
use serde::Serialize;
use crate::common::{civil_from_days, days_from_civil};

// ---------- 9. 紫微斗数 ----------
// 安星口径与 iztro（MIT）默认配置对齐并逐盘验算：年界春节、闰月十五后
// 按下月起命宫、晚子时（23 点）换日。星耀亮度为传统庙陷表。

const ZW_GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
const ZW_ZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
/// 宫名（自命宫逆行）
const PALACE_NAMES: [&str; 12] = [
    "命宫", "兄弟", "夫妻", "子女", "财帛", "疾厄", "迁移", "交友", "官禄", "田宅", "福德", "父母",
];

fn pmod(a: i64, m: i64) -> i64 {
    ((a % m) + m) % m
}

/// 星耀亮度表（庙旺得利平不陷），按宫位索引（寅=0）排列；空串为无亮度
fn brightness(star: &str, p: i64) -> &'static str {
    let t: &[&str; 12] = match star {
        "紫微" => &["旺", "旺", "得", "旺", "庙", "庙", "旺", "旺", "得", "旺", "平", "庙"],
        "天机" => &["得", "旺", "利", "平", "庙", "陷", "得", "旺", "利", "平", "庙", "陷"],
        "太阳" => &["旺", "庙", "旺", "旺", "旺", "得", "得", "平", "不", "陷", "陷", "不"],
        "武曲" => &["得", "利", "庙", "平", "旺", "庙", "得", "利", "庙", "平", "旺", "庙"],
        "天同" => &["利", "平", "平", "庙", "陷", "不", "旺", "平", "平", "庙", "旺", "不"],
        "廉贞" => &["庙", "平", "利", "陷", "平", "利", "庙", "平", "利", "陷", "平", "利"],
        "天府" => &["庙", "得", "庙", "得", "旺", "庙", "得", "旺", "庙", "得", "庙", "庙"],
        "太阴" => &["旺", "陷", "陷", "陷", "不", "不", "利", "旺", "旺", "庙", "庙", "庙"],
        "贪狼" => &["平", "利", "庙", "陷", "旺", "庙", "平", "利", "庙", "陷", "旺", "庙"],
        "巨门" => &["庙", "庙", "陷", "旺", "旺", "不", "庙", "庙", "陷", "旺", "旺", "不"],
        "天相" => &["庙", "陷", "得", "得", "庙", "得", "庙", "陷", "得", "得", "庙", "庙"],
        "天梁" => &["庙", "庙", "庙", "陷", "庙", "旺", "陷", "得", "庙", "陷", "庙", "旺"],
        "七杀" => &["庙", "旺", "庙", "平", "旺", "庙", "庙", "旺", "庙", "平", "旺", "庙"],
        "破军" => &["得", "陷", "旺", "平", "庙", "旺", "得", "陷", "旺", "平", "庙", "旺"],
        "文昌" => &["陷", "利", "得", "庙", "陷", "利", "得", "庙", "陷", "利", "得", "庙"],
        "文曲" => &["平", "旺", "得", "庙", "陷", "旺", "得", "庙", "陷", "旺", "得", "庙"],
        "火星" => &["庙", "利", "陷", "得", "庙", "利", "陷", "得", "庙", "利", "陷", "得"],
        "铃星" => &["庙", "利", "陷", "得", "庙", "利", "陷", "得", "庙", "利", "陷", "得"],
        "擎羊" => &["", "陷", "庙", "", "陷", "庙", "", "陷", "庙", "", "陷", "庙"],
        "陀罗" => &["陷", "", "庙", "陷", "", "庙", "陷", "", "庙", "陷", "", "庙"],
        _ => return "",
    };
    t[p as usize]
}

/// 四化表（按年干）：[化禄， 化权， 化科， 化忌]
fn mutagens(year_gan: &str) -> [&'static str; 4] {
    match year_gan {
        "甲" => ["廉贞", "破军", "武曲", "太阳"],
        "乙" => ["天机", "天梁", "紫微", "太阴"],
        "丙" => ["天同", "天机", "文昌", "廉贞"],
        "丁" => ["太阴", "天同", "天机", "巨门"],
        "戊" => ["贪狼", "太阴", "右弼", "天机"],
        "己" => ["武曲", "贪狼", "天梁", "文曲"],
        "庚" => ["太阳", "武曲", "太阴", "天同"],
        "辛" => ["巨门", "太阳", "文曲", "文昌"],
        "壬" => ["天梁", "紫微", "左辅", "武曲"],
        _ => ["破军", "巨门", "太阴", "贪狼"], // 癸
    }
}

#[derive(Serialize, Clone)]
pub struct ZwStar {
    pub name: String,
    pub kind: String,
    pub brightness: String,
    pub hua: String,
}

#[derive(Serialize)]
pub struct ZwPalace {
    pub branch: String,
    pub ganzhi: String,
    pub name: String,
    pub is_soul: bool,
    pub is_body: bool,
    pub daxian: String,
    pub stars: Vec<ZwStar>,
}

#[derive(Serialize)]
pub struct ZiweiChart {
    pub lunar_text: String,
    pub year_ganzhi: String,
    pub soul_ganzhi: String,
    pub body_ganzhi: String,
    pub ju: String,
    pub ziwei_branch: String,
    pub palaces: Vec<ZwPalace>,
}

/// 紫微排盘：公历出生年月日时分 + 性别（1 男 0 女）。
pub fn ziwei_chart(y: i64, m: i64, d: i64, hour: i64, minute: i64, gender: i64) -> ZiweiChart {
    // 晚子时换日（23 点起按下日算）
    let (ey, em, ed) = if hour >= 23 {
        civil_from_days(days_from_civil(y, m, d) + 1)
    } else {
        (y, m, d)
    };
    let solar = solar::from_ymdhms(ey, em, ed, hour % 24, minute, 0);
    let lunar = solar.get_lunar();
    let year_gz = lunar.get_year_in_gan_zhi();
    let year_gan = year_gz.chars().next().unwrap().to_string();
    let year_zhi = year_gz.chars().nth(1).unwrap().to_string();
    let ygi = ZW_GAN.iter().position(|x| *x == year_gan).unwrap() as i64;
    let yzi = ZW_ZHI.iter().position(|x| *x == year_zhi).unwrap() as i64;
    let raw_month = lunar.get_month();
    let is_leap = raw_month < 0;
    let lm = raw_month.abs();
    let ld = lunar.get_day();
    // 时支（子=0）
    let hz: i64 = if hour >= 23 { 0 } else { ((hour + 1) / 2) % 12 };
    // 命宫用月份索引：闰月且日 > 15 按下月
    let lm0 = (lm - 1) + if is_leap && ld > 15 { 1 } else { 0 };
    let p_soul = pmod(lm0 - hz, 12);
    let p_body = pmod(lm0 + hz, 12);
    // 宫干：五虎遁寅干 + 宫位偏移
    let tiger: i64 = match ygi {
        0 | 5 => 2,  // 甲己 丙寅
        1 | 6 => 4,  // 乙庚 戊寅
        2 | 7 => 6,  // 丙辛 庚寅
        3 | 8 => 8,  // 丁壬 壬寅
        _ => 0,      // 戊癸 甲寅
    };
    let palace_stem = |p: i64| -> i64 { pmod(tiger + p, 10) };
    let palace_branch = |p: i64| -> i64 { pmod(p + 2, 12) };
    // 五行局（命宫干支取数法）
    let soul_stem = palace_stem(p_soul);
    let soul_branch = palace_branch(p_soul);
    let stem_num = soul_stem / 2 + 1;
    let branch_num = (soul_branch % 6) / 2 + 1;
    let mut s = stem_num + branch_num;
    while s > 5 {
        s -= 5;
    }
    let (ju_name, ju_num): (&str, i64) = match s {
        1 => ("木三局", 3),
        2 => ("金四局", 4),
        3 => ("水二局", 2),
        4 => ("火六局", 6),
        _ => ("土五局", 5),
    };
    // 紫微定位
    let q = ju_num;
    let mut k = 0i64;
    while (ld + k) % q != 0 {
        k += 1;
    }
    let quotient = ((ld + k) / q) % 12;
    let p_zw = pmod(quotient - 1 + if k % 2 == 0 { k } else { -k }, 12);
    let p_tf = pmod(12 - p_zw, 12);

    // 星耀落宫：(宫位索引 p, 星名, 类别）
    let mut stars_at: Vec<Vec<ZwStar>> = vec![Vec::new(); 12];
    let hua = mutagens(&year_gan);
    let hua_of = |name: &str| -> String {
        for (i, hn) in hua.iter().enumerate() {
            if *hn == name {
                return ["禄", "权", "科", "忌"][i].to_string();
            }
        }
        String::new()
    };
    let put = |p: i64, name: &str, kind: &str, stars_at: &mut Vec<Vec<ZwStar>>| {
        let p = pmod(p, 12);
        stars_at[p as usize].push(ZwStar {
            name: name.to_string(),
            kind: kind.to_string(),
            brightness: brightness(name, p).to_string(),
            hua: hua_of(name),
        });
    };
    // 紫微系（逆行）
    for (off, name) in [(0, "紫微"), (-1, "天机"), (-3, "太阳"), (-4, "武曲"), (-5, "天同"), (-8, "廉贞")] {
        put(p_zw + off, name, "主星", &mut stars_at);
    }
    // 天府系（顺行）
    for (off, name) in [(0, "天府"), (1, "太阴"), (2, "贪狼"), (3, "巨门"), (4, "天相"), (5, "天梁"), (6, "七杀"), (10, "破军")] {
        put(p_tf + off, name, "主星", &mut stars_at);
    }
    // 辅星（先算地支，再转宫位索引 p = 支 - 2）
    let b2p = |b: i64| pmod(b - 2, 12);
    put(b2p(pmod(10 - hz, 12)), "文昌", "辅星", &mut stars_at);
    put(b2p(pmod(4 + hz, 12)), "文曲", "辅星", &mut stars_at);
    put(2 + lm0, "左辅", "辅星", &mut stars_at);
    put(8 - lm0, "右弼", "辅星", &mut stars_at);
    let (kui, yue) = match ygi {
        0 | 4 | 6 => (1, 7),
        1 | 5 => (0, 8),
        7 => (6, 2),
        2 | 3 => (11, 9),
        _ => (3, 5),
    };
    put(b2p(kui), "天魁", "辅星", &mut stars_at);
    put(b2p(yue), "天钺", "辅星", &mut stars_at);
    let lu_branch = match ygi {
        0 => 2, 1 => 3, 2 | 4 => 5, 3 | 5 => 6, 6 => 8, 7 => 9, 8 => 11, _ => 0,
    };
    put(b2p(lu_branch), "禄存", "辅星", &mut stars_at);
    put(b2p(lu_branch + 1), "擎羊", "煞星", &mut stars_at);
    put(b2p(lu_branch - 1), "陀罗", "煞星", &mut stars_at);
    let ma_branch = match yzi {
        8 | 0 | 4 => 2,
        2 | 6 | 10 => 8,
        5 | 9 | 1 => 11,
        _ => 5,
    };
    put(b2p(ma_branch), "天马", "辅星", &mut stars_at);
    let (huo_start, ling_start) = match yzi {
        2 | 6 | 10 => (1, 3),
        8 | 0 | 4 => (2, 10),
        5 | 9 | 1 => (3, 10),
        _ => (9, 10),
    };
    put(b2p(huo_start + hz), "火星", "煞星", &mut stars_at);
    put(b2p(ling_start + hz), "铃星", "煞星", &mut stars_at);
    put(b2p(pmod(11 - hz, 12)), "地空", "煞星", &mut stars_at);
    put(b2p(pmod(11 + hz, 12)), "地劫", "煞星", &mut stars_at);
    let luan_branch = pmod(3 - yzi, 12);
    put(b2p(luan_branch), "红鸾", "辅星", &mut stars_at);
    put(b2p(luan_branch + 6), "天喜", "辅星", &mut stars_at);

    // 大限：阳男阴女顺行（年支阴阳 × 性别）
    let year_yang = yzi % 2 == 0;
    let forward = year_yang == (gender == 1);
    let mut daxian_of: Vec<String> = vec![String::new(); 12];
    for step in 0..12i64 {
        let p = if forward { pmod(p_soul + step, 12) } else { pmod(p_soul - step, 12) };
        let start = ju_num + step * 10;
        daxian_of[p as usize] = format!("{}-{}", start, start + 9);
    }

    let gz_of = |p: i64| -> String {
        format!("{}{}", ZW_GAN[palace_stem(p) as usize], ZW_ZHI[palace_branch(p) as usize])
    };
    let mut palaces = Vec::new();
    for p in 0..12i64 {
        let name_idx = pmod(p_soul - p, 12);
        palaces.push(ZwPalace {
            branch: ZW_ZHI[palace_branch(p) as usize].to_string(),
            ganzhi: gz_of(p),
            name: PALACE_NAMES[name_idx as usize].to_string(),
            is_soul: p == p_soul,
            is_body: p == p_body,
            daxian: daxian_of[p as usize].clone(),
            stars: stars_at[p as usize].clone(),
        });
    }
    ZiweiChart {
        lunar_text: format!(
            "{}年{}月{}",
            lunar.get_year_in_chinese(),
            lunar.get_month_in_chinese(),
            lunar.get_day_in_chinese()
        ),
        year_ganzhi: year_gz,
        soul_ganzhi: gz_of(p_soul),
        body_ganzhi: gz_of(p_body),
        ju: ju_name.to_string(),
        ziwei_branch: ZW_ZHI[palace_branch(p_zw) as usize].to_string(),
        palaces,
    }
}

// ---------- 紫微流年 ----------

#[derive(Serialize)]
pub struct LiuPalace {
    /// 宫位地支（与本命盘 palaces 同序对应）
    pub branch: String,
    /// 流年宫名（流年命宫 / 流年兄弟 / …）
    pub name: String,
}

#[derive(Serialize)]
pub struct LiuHua {
    pub star: String,
    /// 禄 / 权 / 科 / 忌
    pub hua: String,
    /// 该星所在的本命宫名
    pub palace: String,
    pub branch: String,
}

#[derive(Serialize)]
pub struct LiuNian {
    pub year: i64,
    pub year_ganzhi: String,
    /// 流年命宫地支
    pub soul_branch: String,
    /// 流年命宫所在的本命宫名
    pub soul_palace_name: String,
    pub palaces: Vec<LiuPalace>,
    pub mutagens: Vec<LiuHua>,
}

/// 紫微流年：在本命盘上叠流年一层。流年按农历年干支（公元 4 年甲子起排）：
/// 流年命宫落在流年地支之宫，流年十二宫自此逆行排布，流年四化按流年天干
/// 查本命四化表。口径对照 iztro 的 horoscope(yearly) 验算。
pub fn ziwei_liunian(y: i64, m: i64, d: i64, hour: i64, minute: i64, gender: i64, year: i64) -> LiuNian {
    let chart = ziwei_chart(y, m, d, hour, minute, gender);
    let gan_idx = (year - 4).rem_euclid(10);
    let zhi_idx = (year - 4).rem_euclid(12);
    let year_ganzhi = format!("{}{}", ZW_GAN[gan_idx as usize], ZW_ZHI[zhi_idx as usize]);
    let soul_branch = ZW_ZHI[zhi_idx as usize];
    // 流年十二宫：自流年命宫地支起逆行（地支序递减）
    let palaces: Vec<LiuPalace> = chart
        .palaces
        .iter()
        .map(|p| {
            let b = ZW_ZHI.iter().position(|&z| z == p.branch).unwrap() as i64;
            let off = pmod(zhi_idx - b, 12);
            LiuPalace {
                branch: p.branch.clone(),
                name: format!("流年{}", PALACE_NAMES[off as usize]),
            }
        })
        .collect();
    let soul_palace_name = chart
        .palaces
        .iter()
        .find(|p| p.branch == soul_branch)
        .map(|p| p.name.clone())
        .unwrap_or_default();
    // 流年四化：按流年天干查表，并定位每颗化星所在的本命宫
    let hua = mutagens(ZW_GAN[gan_idx as usize]);
    let mutagens: Vec<LiuHua> = hua
        .iter()
        .enumerate()
        .map(|(i, star)| {
            let (palace, branch) = chart
                .palaces
                .iter()
                .find_map(|p| {
                    if p.stars.iter().any(|s| s.name == *star) {
                        Some((p.name.clone(), p.branch.clone()))
                    } else {
                        None
                    }
                })
                .unwrap_or_default();
            LiuHua {
                star: star.to_string(),
                hua: ["禄", "权", "科", "忌"][i].to_string(),
                palace,
                branch,
            }
        })
        .collect();
    LiuNian {
        year,
        year_ganzhi,
        soul_branch: soul_branch.to_string(),
        soul_palace_name,
        palaces,
        mutagens,
    }
}
