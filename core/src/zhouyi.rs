//! 周易抽卦：金钱卦法——三枚铜钱摇六次，自下而上成卦。
//! 每爻三钱点数和：6 老阴（变）、7 少阳、8 少阴、9 老阳（变）。

use serde::Serialize;
use crate::common::hash64;
use crate::hexagram::{self, Hexagram};

#[derive(Serialize, Clone)]
pub struct HexView {
    pub n: u32,
    pub name: String,
    pub full: String,
    pub guaci: String,
    pub xiang: String,
    pub dayi: String,
}

impl From<&Hexagram> for HexView {
    fn from(h: &Hexagram) -> Self {
        HexView {
            n: h.n,
            name: h.name.clone(),
            full: h.full.clone(),
            guaci: h.guaci.clone(),
            xiang: h.xiang.clone(),
            dayi: h.dayi.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct YaoLine {
    /// 铜钱点数和 6/7/8/9
    pub value: u8,
    pub yang: bool,
    pub changing: bool,
    /// 爻名（初九、六二…）
    pub name: String,
    /// 爻辞（仅变爻给出，其余为空串）
    pub text: String,
}

#[derive(Serialize)]
pub struct ZhouyiDraw {
    /// 六爻（自下而上）
    pub lines: Vec<YaoLine>,
    pub ben: HexView,
    /// 变卦（无变爻时为空）
    pub bian: Option<HexView>,
    /// 乾坤六爻皆变时的用九 / 用六（其余为空）
    pub yong: Option<String>,
}

/// 由六爻点数（自下而上，6..=9）成卦。纯函数，自检锚点直接调用。
pub fn build_from_values(values: [u8; 6]) -> ZhouyiDraw {
    let mut bits = [0u8; 6];
    let mut changing = [false; 6];
    for i in 0..6 {
        let v = values[i];
        bits[i] = if v == 7 || v == 9 { 1 } else { 0 };
        changing[i] = v == 6 || v == 9;
    }
    let ben = hexagram::by_lines(bits);
    let lines: Vec<YaoLine> = (0..6)
        .map(|i| YaoLine {
            value: values[i],
            yang: bits[i] == 1,
            changing: changing[i],
            name: hexagram::yao_name(i, bits[i] == 1),
            text: if changing[i] { ben.yao[i].clone() } else { String::new() },
        })
        .collect();
    let any_change = changing.iter().any(|&c| c);
    let bian = if any_change {
        let mut nb = bits;
        for i in 0..6 {
            if changing[i] {
                nb[i] ^= 1;
            }
        }
        Some(HexView::from(hexagram::by_lines(nb)))
    } else {
        None
    };
    let yong = if changing.iter().all(|&c| c) { ben.yong.clone() } else { None };
    ZhouyiDraw { lines, ben: HexView::from(ben), bian, yong }
}

/// 抽卦：种子由页面传入（每次点击取新随机数），核心内以哈希模拟掷钱。
pub fn zhouyi_draw(seed: i64) -> ZhouyiDraw {
    let mut values = [0u8; 6];
    for i in 0..6 {
        let h = hash64((seed as u64) ^ (i as u64).wrapping_mul(0x9E3779B97F4A7C15));
        let mut sum = 0u8;
        for j in 0..3 {
            sum += 2 + ((h >> (j * 7)) & 1) as u8;
        }
        values[i] = sum;
    }
    build_from_values(values)
}
