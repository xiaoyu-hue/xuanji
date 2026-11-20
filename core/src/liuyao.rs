//! 六爻：金钱卦起卦后「装卦」——定卦宫与世应、纳甲装地支、
//! 按宫五行配六亲、按日干起六神、按日干支标旬空，并给出变卦装卦。
//! 本版做到装卦为止（旺衰断卦不在此列），页面会如实标注。

use lunar_rust::{lunar::LunarRefHelper, solar::{self, SolarRefHelper}};
use serde::Serialize;
use crate::hexagram::{self, Hexagram};
use crate::zhouyi::HexView;

const ZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
const GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
const SHEN: [&str; 6] = ["青龙", "朱雀", "勾陈", "螣蛇", "白虎", "玄武"];

fn zhi_wuxing(z: &str) -> &'static str {
    match z {
        "子" | "亥" => "水",
        "寅" | "卯" => "木",
        "巳" | "午" => "火",
        "申" | "酉" => "金",
        _ => "土",
    }
}

/// 纳甲：每卦内卦（初~三）/外卦（四~上）所装地支（自下而上各三）
fn najia(tri: &str) -> ([&'static str; 3], [&'static str; 3]) {
    match tri {
        "乾" => (["子", "寅", "辰"], ["午", "申", "戌"]),
        "坤" => (["未", "巳", "卯"], ["丑", "亥", "酉"]),
        "震" => (["子", "寅", "辰"], ["午", "申", "戌"]),
        "巽" => (["丑", "亥", "酉"], ["未", "巳", "卯"]),
        "坎" => (["寅", "辰", "午"], ["申", "戌", "子"]),
        "离" => (["卯", "丑", "亥"], ["酉", "未", "巳"]),
        "艮" => (["辰", "午", "申"], ["戌", "子", "寅"]),
        _ => (["巳", "卯", "丑"], ["亥", "酉", "未"]), // 兑
    }
}

/// 六亲：宫五行 p 与爻支五行 w 的关系
fn liuqin(p: &str, w: &str) -> &'static str {
    let gen = |a: &str, b: &str| matches!((a, b), ("木", "火") | ("火", "土") | ("土", "金") | ("金", "水") | ("水", "木"));
    let ctl = |a: &str, b: &str| matches!((a, b), ("木", "土") | ("土", "水") | ("水", "火") | ("火", "金") | ("金", "木"));
    if w == p {
        "兄弟"
    } else if gen(w, p) {
        "父母"
    } else if gen(p, w) {
        "子孙"
    } else if ctl(w, p) {
        "官鬼"
    } else {
        "妻财"
    }
}

/// 八宫归属：由宫主卦逐爻翻变生成八宫序列（八纯→一世→…→五世→游魂→归魂），
/// 返回（宫名， 世位 1..=6）。生成规则即传统八宫卦序，无需抄表。
fn palace_of(lines: [u8; 6]) -> (String, u8) {
    const SHI: [u8; 8] = [6, 1, 2, 3, 4, 5, 4, 3];
    for (name, tri_lines) in hexagram::TRIGRAMS.iter().map(|t| (t.0, t.1)) {
        let mut cur = [tri_lines[0], tri_lines[1], tri_lines[2], tri_lines[0], tri_lines[1], tri_lines[2]];
        for step in 0..8 {
            if cur == lines {
                return (name.to_string(), SHI[step]);
            }
            match step {
                0..=4 => cur[step] ^= 1,      // 一世~五世：逐爻自下而上翻
                5 => cur[3] ^= 1,             // 游魂：四爻翻回
                _ => {                        // 归魂：内卦三爻翻回
                    cur[0] ^= 1;
                    cur[1] ^= 1;
                    cur[2] ^= 1;
                }
            }
        }
    }
    unreachable!("任何卦必归一宫")
}

#[derive(Serialize)]
pub struct LyLine {
    pub yang: bool,
    pub changing: bool,
    pub name: String,
    pub zhi: String,
    pub wuxing: String,
    pub liuqin: String,
    pub shen: String,
    pub is_shi: bool,
    pub is_ying: bool,
    pub kong: bool,
}

#[derive(Serialize)]
pub struct LyBianLine {
    pub zhi: String,
    pub wuxing: String,
    pub liuqin: String,
    pub changing: bool,
}

#[derive(Serialize)]
pub struct LiuyaoResult {
    pub ben: HexView,
    pub bian: Option<HexView>,
    pub palace: String,
    pub palace_wuxing: String,
    pub shi: u8,
    pub ying: u8,
    pub day_ganzhi: String,
    pub yue_jian: String,
    pub kong_wang: [String; 2],
    /// 本卦六爻（自下而上）
    pub lines: Vec<LyLine>,
    /// 变卦六爻装卦（自下而上；无变爻时为空）
    pub bian_lines: Option<Vec<LyBianLine>>,
}

fn zhi_of(h: &Hexagram) -> [String; 6] {
    // najia 返回 (内卦三支, 外卦三支)：下卦取其内卦、上卦取其外卦
    let lo = najia(&h.lower).0;
    let up = najia(&h.upper).1;
    [
        lo[0].into(), lo[1].into(), lo[2].into(),
        up[0].into(), up[1].into(), up[2].into(),
    ]
}

/// 由六爻点数装卦（纯函数路径：日期只决定日干支/月建/旬空）。
pub fn liuyao_from_values(values: [u8; 6], y: i64, m: i64, d: i64) -> LiuyaoResult {
    let mut bits = [0u8; 6];
    let mut changing = [false; 6];
    for i in 0..6 {
        bits[i] = if values[i] == 7 || values[i] == 9 { 1 } else { 0 };
        changing[i] = values[i] == 6 || values[i] == 9;
    }
    let ben_ref = hexagram::by_lines(bits);
    let ben = HexView::from(ben_ref);
    let (palace, shi) = palace_of(bits);
    let ying = if shi <= 3 { shi + 3 } else { shi - 3 };
    let palace_wx = hexagram::trigram_by_name(&palace).unwrap().1.to_string();

    let solar = solar::from_ymdhms(y, m, d, 12, 0, 0);
    let lunar = solar.get_lunar();
    let day_gz = lunar.get_day_in_gan_zhi();
    let day_gan = day_gz.chars().next().unwrap().to_string();
    let day_zhi = day_gz.chars().nth(1).unwrap().to_string();
    let month_gz = lunar.get_month_in_gan_zhi();
    let yue_jian = month_gz.chars().nth(1).unwrap().to_string();
    // 旬空：旬首支 = 日支序 - 日干序（模 12），空亡为旬首后第 11、12 支
    let g = GAN.iter().position(|&x| x == day_gan).unwrap() as i64;
    let z = ZHI.iter().position(|&x| x == day_zhi).unwrap() as i64;
    let xun_head = (z - g).rem_euclid(12);
    let kong_wang = [
        ZHI[((xun_head + 10) % 12) as usize].to_string(),
        ZHI[((xun_head + 11) % 12) as usize].to_string(),
    ];
    // 六神起例：甲乙青龙、丙丁朱雀、戊勾陈、己螣蛇、庚辛白虎、壬癸玄武
    let shen_start = match day_gan.as_str() {
        "甲" | "乙" => 0,
        "丙" | "丁" => 1,
        "戊" => 2,
        "己" => 3,
        "庚" | "辛" => 4,
        _ => 5,
    };
    let zhis = zhi_of(ben_ref);
    let lines: Vec<LyLine> = (0..6)
        .map(|i| {
            let w = zhi_wuxing(&zhis[i]);
            LyLine {
                yang: bits[i] == 1,
                changing: changing[i],
                name: hexagram::yao_name(i, bits[i] == 1),
                zhi: zhis[i].clone(),
                wuxing: w.to_string(),
                liuqin: liuqin(&palace_wx, w).to_string(),
                shen: SHEN[(shen_start + i) % 6].to_string(),
                is_shi: (i as u8 + 1) == shi,
                is_ying: (i as u8 + 1) == ying,
                kong: kong_wang.contains(&zhis[i]),
            }
        })
        .collect();
    let (bian, bian_lines) = if changing.iter().any(|&c| c) {
        let mut nb = bits;
        for i in 0..6 {
            if changing[i] {
                nb[i] ^= 1;
            }
        }
        let bref = hexagram::by_lines(nb);
        let bz = zhi_of(bref);
        let bl: Vec<LyBianLine> = (0..6)
            .map(|i| {
                let w = zhi_wuxing(&bz[i]);
                LyBianLine {
                    zhi: bz[i].clone(),
                    wuxing: w.to_string(),
                    liuqin: liuqin(&palace_wx, w).to_string(),
                    changing: changing[i],
                }
            })
            .collect();
        (Some(HexView::from(bref)), Some(bl))
    } else {
        (None, None)
    };
    LiuyaoResult {
        ben,
        bian,
        palace,
        palace_wuxing: palace_wx,
        shi,
        ying,
        day_ganzhi: day_gz,
        yue_jian,
        kong_wang,
        lines,
        bian_lines,
    }
}

/// 六爻起卦：种子掷钱 + 起卦日期（定日干支、月建、旬空）。
pub fn liuyao_cast(seed: i64, y: i64, m: i64, d: i64) -> LiuyaoResult {
    liuyao_from_values(hexagram::coin_values(seed), y, m, d)
}
