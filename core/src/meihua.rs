//! 梅花易数：时间起卦——以农历年月日时数起卦（邵雍法）。
//! 上卦 =(年支序 + 月 + 日) % 8，下卦再加时支序 % 8，动爻以总数 % 6
//! （余 0 作满数）。动爻所在之卦为用，另一卦为体，再看体用五行生克。

use lunar_rust::{lunar::LunarRefHelper, solar::{self, SolarRefHelper}};
use serde::Serialize;
use crate::hexagram::{self, Hexagram};
use crate::zhouyi::HexView;

#[derive(Serialize)]
pub struct MeihuaResult {
    pub lunar_text: String,
    /// 本卦六爻（自下而上，阳=1），页面绘图用
    pub ben_lines: Vec<u8>,
    pub ben: HexView,
    /// 动爻位（1 初爻 ~ 6 上爻）
    pub dong_yao: u8,
    pub dong_name: String,
    pub dong_text: String,
    pub bian: HexView,
    pub ti_gua: String,
    pub yong_gua: String,
    /// 体用关系：用生体 / 体克用 / 比和 / 体生用 / 用克体
    pub relation: String,
    /// 本站原创断语（老话式点缀）
    pub verdict: String,
}

/// 五行生：木→火→土→金→水→木；克：木克土、土克水、水克火、火克金、金克木
fn generates(a: &str, b: &str) -> bool {
    matches!((a, b), ("木", "火") | ("火", "土") | ("土", "金") | ("金", "水") | ("水", "木"))
}
fn controls(a: &str, b: &str) -> bool {
    matches!((a, b), ("木", "土") | ("土", "水") | ("水", "火") | ("火", "金") | ("金", "木"))
}

/// 梅花时间起卦：公历年月日 + 小时（分钟不入卦）。
pub fn meihua(y: i64, m: i64, d: i64, hour: i64) -> MeihuaResult {
    let solar = solar::from_ymdhms(y, m, d, hour, 0, 0);
    let lunar = solar.get_lunar();
    let year_gz = lunar.get_year_in_gan_zhi();
    let year_zhi = year_gz.chars().nth(1).unwrap().to_string();
    const ZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
    let year_num = ZHI.iter().position(|&z| z == year_zhi).unwrap() as i64 + 1;
    let lm = lunar.get_month().abs();
    let ld = lunar.get_day();
    let hour_num = (((hour + 1) / 2) % 12) + 1; // 子时=1 … 亥时=12（23 点作子时）
    let s1 = year_num + lm + ld;
    let s2 = s1 + hour_num;
    let rem8 = |s: i64| if s % 8 == 0 { 8 } else { s % 8 };
    let upper = hexagram::trigram_by_num(rem8(s1) as u8);
    let lower = hexagram::trigram_by_num(rem8(s2) as u8);
    let dong = if s2 % 6 == 0 { 6 } else { s2 % 6 } as u8;
    let ben_ref: &Hexagram = hexagram::by_trigrams(upper, lower);
    let ben = HexView::from(ben_ref);
    let ben_bits = hexagram::lines_of(ben_ref);
    let mut bits = ben_bits;
    bits[(dong - 1) as usize] ^= 1;
    let bian = HexView::from(hexagram::by_lines(bits));
    // 体用：动爻在下卦则下为用、上为体；在上卦则上为用、下为体
    let (ti_name, yong_name) = if dong <= 3 {
        (upper, lower)
    } else {
        (lower, upper)
    };
    let ti_wx = hexagram::trigram_by_name(ti_name).unwrap().1;
    let yong_wx = hexagram::trigram_by_name(yong_name).unwrap().1;
    let (relation, verdict) = if ti_wx == yong_wx {
        ("比和", "内外同气，顺其自然则吉，忌强行折腾。")
    } else if generates(yong_wx, ti_wx) {
        ("用生体", "外力相助之象，所问之事得人扶持，顺势可成。")
    } else if controls(ti_wx, yong_wx) {
        ("体克用", "自家占着主动，事情在掌控之中，只是要费些力气。")
    } else if generates(ti_wx, yong_wx) {
        ("体生用", "自家耗着元气去就事，成事要付出代价，量力而行。")
    } else {
        ("用克体", "外头压力压着自家，眼下宜守不宜攻，等时节转换。")
    };
    MeihuaResult {
        ben_lines: ben_bits.to_vec(),
        lunar_text: format!(
            "{}年{}月{} · {}时",
            lunar.get_year_in_chinese(),
            lunar.get_month_in_chinese(),
            lunar.get_day_in_chinese(),
            ZHI[((hour + 1) / 2 % 12) as usize]
        ),
        dong_name: hexagram::yao_name((dong - 1) as usize, ben_bits[(dong - 1) as usize] == 1),
        dong_text: ben_ref.yao[(dong - 1) as usize].clone(),
        ben,
        dong_yao: dong,
        bian,
        ti_gua: ti_name.to_string(),
        yong_gua: yong_name.to_string(),
        relation: relation.to_string(),
        verdict: verdict.to_string(),
    }
}
