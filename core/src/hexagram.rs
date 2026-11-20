//! 六十四卦数据层：周易抽卦与梅花易数共用的基础数据，只做数据与查表，
//! 不含任何占断逻辑（两个体系模块都建立在本层之上，互不引用对方）。
//! 经文（卦辞/大象/爻辞）为《周易》公有领域原文；一句话大意为本站原创。

use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Deserialize, Clone)]
pub struct Hexagram {
    pub n: u32,
    pub name: String,
    pub full: String,
    pub upper: String,
    pub lower: String,
    pub guaci: String,
    pub xiang: String,
    pub yao: Vec<String>,
    #[serde(default)]
    pub yong: Option<String>,
    pub dayi: String,
}

const HEXAGRAMS_JSON: &str = include_str!("../../content/hexagrams.json");

pub fn hexagrams() -> &'static Vec<Hexagram> {
    static CACHE: OnceLock<Vec<Hexagram>> = OnceLock::new();
    CACHE.get_or_init(|| serde_json::from_str(HEXAGRAMS_JSON).expect("hexagrams.json 格式错误"))
}

/// 八卦：(卦名, 三爻自下而上 [阳=1], 五行, 先天数)
pub const TRIGRAMS: [(&str, [u8; 3], &str, u8); 8] = [
    ("乾", [1, 1, 1], "金", 1),
    ("兑", [1, 1, 0], "金", 2),
    ("离", [1, 0, 1], "火", 3),
    ("震", [1, 0, 0], "木", 4),
    ("巽", [0, 1, 1], "木", 5),
    ("坎", [0, 1, 0], "水", 6),
    ("艮", [0, 0, 1], "土", 7),
    ("坤", [0, 0, 0], "土", 8),
];

/// 按卦名取八卦 (三爻， 五行， 先天数）
pub fn trigram_by_name(name: &str) -> Option<([u8; 3], &'static str, u8)> {
    TRIGRAMS.iter().find(|t| t.0 == name).map(|t| (t.1, t.2, t.3))
}

/// 按先天数取八卦名（1 乾 2 兑 3 离 4 震 5 巽 6 坎 7 艮 8 坤）
pub fn trigram_by_num(num: u8) -> &'static str {
    TRIGRAMS.iter().find(|t| t.3 == num).map(|t| t.0).unwrap_or("坤")
}

/// 一卦的六爻（自下而上，阳=1）：下卦三爻 + 上卦三爻
pub fn lines_of(h: &Hexagram) -> [u8; 6] {
    let lo = trigram_by_name(&h.lower).unwrap().0;
    let up = trigram_by_name(&h.upper).unwrap().0;
    [lo[0], lo[1], lo[2], up[0], up[1], up[2]]
}

/// 按六爻（自下而上）查卦
pub fn by_lines(lines: [u8; 6]) -> &'static Hexagram {
    let lo = TRIGRAMS.iter().find(|t| t.1 == [lines[0], lines[1], lines[2]]).unwrap();
    let up = TRIGRAMS.iter().find(|t| t.1 == [lines[3], lines[4], lines[5]]).unwrap();
    by_trigrams(up.0, lo.0)
}

/// 按上下卦查卦
pub fn by_trigrams(upper: &str, lower: &str) -> &'static Hexagram {
    hexagrams()
        .iter()
        .find(|h| h.upper == upper && h.lower == lower)
        .expect("上下卦组合必有一卦")
}

/// 按文王序查卦
pub fn by_num(n: u32) -> &'static Hexagram {
    hexagrams().iter().find(|h| h.n == n).expect("卦序 1..=64")
}

/// 爻名：位（0 初 ~ 5 上）+ 阴阳 → 初九 / 六二 / 九三 / 上六 …
pub fn yao_name(pos: usize, yang: bool) -> String {
    let nine_six = if yang { "九" } else { "六" };
    match pos {
        0 => format!("初{}", nine_six),
        5 => format!("上{}", nine_six),
        _ => format!("{}{}", nine_six, ["二", "三", "四", "五"][pos - 1]),
    }
}
