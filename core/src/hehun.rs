//! 合婚（民间简法：生肖 + 纳音 + 日主）。

use lunar_rust::{
    eight_char::EightCharRefHelper,
    lunar::LunarRefHelper,
    solar::{self, SolarRefHelper},
};
use serde::Serialize;

// ---------- 7. 合婚（民间简法：生肖 + 纳音 + 日主） ----------

#[derive(Serialize)]
pub struct MatchFactor {
    pub name: String,
    pub result: String,
    pub delta: i64,
    pub note: String,
}

#[derive(Serialize)]
pub struct MatchResult {
    pub a_zodiac: String,
    pub b_zodiac: String,
    pub a_nayin: String,
    pub b_nayin: String,
    pub a_day_gan: String,
    pub b_day_gan: String,
    pub score: i64,
    pub grade: String,
    pub factors: Vec<MatchFactor>,
}

const ZODIACS: [&str; 12] = ["鼠", "牛", "虎", "兔", "龙", "蛇", "马", "羊", "猴", "鸡", "狗", "猪"];

fn pair_in(pairs: &[(usize, usize)], a: usize, b: usize) -> bool {
    pairs.iter().any(|&(x, y)| (x == a && y == b) || (x == b && y == a))
}

/// 五行关系：0 比和、1 a 生 b、2 b 生 a、3 a 克 b、4 b 克 a
/// 五行序：金0 木1 水2 火3 土4；生序 木→火→土→金→水→木
fn wx_relation(a: usize, b: usize) -> i32 {
    if a == b {
        return 0;
    }
    // 生：木(1)生火(3)、火生土(4)、土生金(0)、金生水(2)、水生木(1)
    const SHENG: [(usize, usize); 5] = [(1, 3), (3, 4), (4, 0), (0, 2), (2, 1)];
    // 克：木克土、土克水、水克火、火克金、金克木
    const KE: [(usize, usize); 5] = [(1, 4), (4, 2), (2, 3), (3, 0), (0, 1)];
    if SHENG.contains(&(a, b)) {
        1
    } else if SHENG.contains(&(b, a)) {
        2
    } else if KE.contains(&(a, b)) {
        3
    } else {
        4
    }
}

fn wx_index(name: &str) -> usize {
    match name {
        "金" => 0,
        "木" => 1,
        "水" => 2,
        "火" => 3,
        _ => 4,
    }
}

/// 合婚：两人公历出生年月日时分（时分主要用于日柱，子时口径同八字）。
pub fn hehun(
    ay: i64, am: i64, ad: i64, ah: i64, amin: i64,
    by: i64, bm: i64, bd: i64, bh: i64, bmin: i64,
) -> MatchResult {
    let la = solar::from_ymdhms(ay, am, ad, ah, amin, 0).get_lunar();
    let lb = solar::from_ymdhms(by, bm, bd, bh, bmin, 0).get_lunar();
    let za = la.get_year_sheng_xiao();
    let zb = lb.get_year_sheng_xiao();
    let ia = ZODIACS.iter().position(|x| *x == za).unwrap();
    let ib = ZODIACS.iter().position(|x| *x == zb).unwrap();
    let mut factors: Vec<MatchFactor> = Vec::new();
    let mut score: i64 = 50;

    // 1) 生肖
    const LIUHE: [(usize, usize); 6] = [(0, 1), (2, 11), (3, 10), (4, 9), (5, 8), (6, 7)];
    const SANHE: [(usize, usize); 12] = [
        (8, 0), (8, 4), (0, 4), // 申子辰
        (11, 3), (11, 7), (3, 7), // 亥卯未
        (2, 6), (2, 10), (6, 10), // 寅午戌
        (5, 9), (5, 1), (9, 1), // 巳酉丑
    ];
    const CHONG: [(usize, usize); 6] = [(0, 6), (1, 7), (2, 8), (3, 9), (4, 10), (5, 11)];
    const HAI: [(usize, usize); 6] = [(0, 7), (1, 6), (2, 5), (3, 4), (8, 11), (9, 10)];
    let (zres, zdelta, znote) = if ia == ib {
        ("同肖", 5, "同生肖：脾气路数相近，容易懂彼此，也容易一起犯倔。")
    } else if pair_in(&LIUHE, ia, ib) {
        ("六合", 30, "生肖六合：传统里最亲近的一组搭配，性情相投、相处省力。")
    } else if pair_in(&SANHE, ia, ib) {
        ("三合", 25, "生肖三合：目标和节奏容易合拍，是能一起成事的组合。")
    } else if pair_in(&CHONG, ia, ib) {
        ("相冲", -25, "生肖相冲：脾气都直，遇事容易顶牛；多沟通、留台阶是关键。")
    } else if pair_in(&HAI, ia, ib) {
        ("相害", -15, "生肖相害：容易在小事上互相别扭，话说开就没事。")
    } else if (ia == 0 && ib == 3) || (ia == 3 && ib == 0) {
        ("相刑", -10, "生肖相刑（无礼之刑）：相处要多讲礼数，客气一点反而长久。")
    } else {
        ("平和", 10, "生肖无冲无合：没有天生的加减分，感情靠日常经营。")
    };
    score += zdelta;
    factors.push(MatchFactor {
        name: "生肖配对".into(),
        result: format!("{za} × {zb} · {zres}"),
        delta: zdelta,
        note: znote.into(),
    });

    // 2) 年柱纳音五行
    let na = la.get_year_na_yin();
    let nb = lb.get_year_na_yin();
    let wa = wx_index(&na.chars().last().unwrap().to_string());
    let wb = wx_index(&nb.chars().last().unwrap().to_string());
    let (nres, ndelta, nnote) = match wx_relation(wa, wb) {
        0 => ("比和", 12, "纳音同行：根基与生活节奏接近，日子容易过到一处去。"),
        1 | 2 => ("相生", 18, "纳音相生：一方愿意扶持一方，传统里是越过越旺的组合。"),
        _ => ("相克", -10, "纳音相克：花钱与过日子的观念可能不同，早商量好家里的账。"),
    };
    score += ndelta;
    factors.push(MatchFactor {
        name: "纳音五行".into(),
        result: format!("{na} × {nb} · {nres}"),
        delta: ndelta,
        note: nnote.into(),
    });

    // 3) 日主天干
    let ga = la.get_eight_char().get_day_gan();
    let gb = lb.get_eight_char().get_day_gan();
    const GAN5: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
    let gia = GAN5.iter().position(|x| *x == ga).unwrap();
    let gib = GAN5.iter().position(|x| *x == gb).unwrap();
    // 天干五行：甲乙木、丙丁火、戊己土、庚辛金、壬癸水
    let gwx = |i: usize| -> usize { [1, 1, 3, 3, 4, 4, 0, 0, 2, 2][i] };
    let is_wuhe = matches!(
        (gia, gib),
        (0, 5) | (5, 0) | (1, 6) | (6, 1) | (2, 7) | (7, 2) | (3, 8) | (8, 3) | (4, 9) | (9, 4)
    );
    let (gres, gdelta, gnote) = if is_wuhe {
        ("天干五合", 20, "日主天干五合：两人之间有天然的亲近感，传统里叫「有情之合」。")
    } else {
        match wx_relation(gwx(gia), gwx(gib)) {
            0 => ("比和", 8, "日主同行：想法容易一致，做决定时少拉扯。"),
            1 | 2 => ("相生", 10, "日主相生：相处里多是鼓励和托举，关系越处越稳。"),
            _ => ("相克", -8, "日主相克：都想说了算，事先分好工、少翻旧账。"),
        }
    };
    score += gdelta;
    factors.push(MatchFactor {
        name: "日主天干".into(),
        result: format!("{ga} × {gb} · {gres}"),
        delta: gdelta,
        note: gnote.into(),
    });

    let score = score.clamp(5, 98);
    let grade = if score >= 85 {
        "天作之合"
    } else if score >= 70 {
        "佳偶良缘"
    } else if score >= 55 {
        "和合美满"
    } else if score >= 40 {
        "渐入佳境"
    } else {
        "需要磨合"
    };
    MatchResult {
        a_zodiac: za,
        b_zodiac: zb,
        a_nayin: na,
        b_nayin: nb,
        a_day_gan: ga,
        b_day_gan: gb,
        score,
        grade: grade.into(),
        factors,
    }
}
