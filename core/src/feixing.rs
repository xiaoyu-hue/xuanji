//! 九宫飞星：流年紫白飞布九宫（玄空飞星的年盘）。
//! 中宫星按年递减（九年一轮），入中后按洛书轨迹顺飞八宫。

use serde::Serialize;

#[derive(Serialize)]
pub struct FxPalace {
    /// 方位：东南 / 南 / 西南 / 东 / 中 / 西 / 东北 / 北 / 西北
    pub dir: String,
    /// 宫名（卦）：巽 / 离 / 坤 / 震 / 中 / 兑 / 艮 / 坎 / 乾
    pub palace: String,
    pub star: u8,
    /// 星名：一白贪狼 …
    pub star_name: String,
    pub wuxing: String,
    /// 吉 / 凶
    pub luck: String,
    /// 星性一句话（本站原创）
    pub note: String,
}

#[derive(Serialize)]
pub struct Feixing {
    pub year: i64,
    pub ganzhi: String,
    pub center: u8,
    /// 按盘面行序：[巽 离 坤 / 震 中 兑 / 艮 坎 乾]（上南下北）
    pub palaces: Vec<FxPalace>,
}

/// 九星：(星名， 五行， 吉凶， 星性）
const STARS: [(&str, &str, &str, &str); 9] = [
    ("一白贪狼", "水", "吉", "事业、人缘与桃花之星，利谋事进取。"),
    ("二黑巨门", "土", "凶", "病符星，主健康损耗，宜静养忌扰动。"),
    ("三碧禄存", "木", "凶", "是非星，主口舌争讼，宜和气忍让。"),
    ("四绿文曲", "木", "吉", "文昌星，利学业考试与文艺创作。"),
    ("五黄廉贞", "土", "凶", "灾煞星，诸事不宜，忌动土兴工。"),
    ("六白武曲", "金", "吉", "权贵星，利事业权威与偏财。"),
    ("七赤破军", "金", "凶", "破财星，防盗失与口舌刀兵。"),
    ("八白左辅", "土", "吉", "正财星，利置业积蓄与正财。"),
    ("九紫右弼", "火", "吉", "喜庆星，利婚嫁喜事与名声晋升。"),
];

/// 洛书顺飞轨迹（自中宫起）：中 → 乾（西北）→ 兑（西）→ 艮（东北）→
/// 离（南）→ 坎（北）→ 坤（西南）→ 震（东）→ 巽（东南）
const FLIGHT: [&str; 9] = ["中", "乾", "兑", "艮", "离", "坎", "坤", "震", "巽"];
/// 盘面行序（上南下北）与方位
const DISPLAY: [(&str, &str); 9] = [
    ("巽", "东南"), ("离", "南"), ("坤", "西南"),
    ("震", "东"), ("中", "中"), ("兑", "西"),
    ("艮", "东北"), ("坎", "北"), ("乾", "西北"),
];

const GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
const ZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

/// 流年飞星：year 为干支年（以立春为界，页面负责换算）。
pub fn feixing(year: i64) -> Feixing {
    // 中宫星：以 2026 年一白入中为锚，逐年递减、九年一轮
    let center = (((2026 - year) % 9 + 9) % 9 + 1) as u8;
    let mut star_at: std::collections::HashMap<&str, u8> = std::collections::HashMap::new();
    for (i, p) in FLIGHT.iter().enumerate() {
        let s = (center - 1 + i as u8) % 9 + 1;
        star_at.insert(p, s);
    }
    let palaces = DISPLAY
        .iter()
        .map(|(name, dir)| {
            let star = star_at[name];
            let (sn, wx, luck, note) = STARS[(star - 1) as usize];
            FxPalace {
                dir: dir.to_string(),
                palace: name.to_string(),
                star,
                star_name: sn.to_string(),
                wuxing: wx.to_string(),
                luck: luck.to_string(),
                note: note.to_string(),
            }
        })
        .collect();
    Feixing {
        year,
        ganzhi: format!(
            "{}{}",
            GAN[((year - 4).rem_euclid(10)) as usize],
            ZHI[((year - 4).rem_euclid(12)) as usize]
        ),
        center,
        palaces,
    }
}
