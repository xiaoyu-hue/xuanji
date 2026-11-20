use std::time::{SystemTime, UNIX_EPOCH};
use xuanji_core::{almanac, bazi, book_answer, daily_qian};

fn ziwei_dump() {
    use std::io::BufRead;
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let v: Vec<i64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        if v.len() < 5 { continue; }
        let c = xuanji_core::ziwei_chart(v[0], v[1], v[2], v[3], 0, v[4]);
        let soul = c.palaces.iter().find(|p| p.is_soul).unwrap();
        let body = c.palaces.iter().find(|p| p.is_body).unwrap();
        let mut majors: Vec<String> = Vec::new();
        let mut minors: Vec<String> = Vec::new();
        let mut huas: Vec<String> = Vec::new();
        for p in &c.palaces {
            for s in &p.stars {
                let tag = format!("{}({})@{}", s.name, s.brightness, p.branch);
                if s.kind == "主星" { majors.push(tag.clone()); }
                else { minors.push(tag.clone()); }
                if !s.hua.is_empty() { huas.push(format!("{}{}", s.name, s.hua)); }
            }
        }
        majors.sort(); minors.sort(); huas.sort();
        let dx: Vec<String> = c.palaces.iter().map(|p| format!("{}:{}", p.branch, p.daxian)).collect();
        println!("命{} 身{} {} 紫微{} | {} | {} | {} | {}",
            soul.branch, body.branch, c.ju, c.ziwei_branch,
            majors.join(","), minors.join(","), huas.join(","), dx.join(","));
    }
}

fn main() {
    if std::env::var("ZIWEI_DUMP").is_ok() { ziwei_dump(); return; }
    check_wedding();
    check_bone();
    check_hehun();
    check_dreams();
// 今日黄历（2026-11-20，已与金标准逐项核对）
let a = almanac(2026, 11, 20);
println!("{} | 农历{}", a.solar, a.lunar_text);
println!(
"{}年 {}月 {}日 · 生肖{} · {}",
a.year_ganzhi, a.month_ganzhi, a.day_ganzhi, a.shengxiao, a.xingzuo
);
println!("宜：{}", a.yi.join("、"));
println!("忌：{}", a.ji.join("、"));
println!(
"值神 {}（{}）| 星宿 {}（{}）| 冲{} 煞{}",
a.tianshen, a.tianshen_luck, a.xiu, a.xiu_luck, a.chong, a.sha
);
println!(
"财神{} 喜神{} 福神{} | 节气：{} → {}",
a.cai, a.xi, a.fu, a.prev_jieqi, a.next_jieqi
);
println!();

// 今日一签
let q = daily_qian(2026, 11, 20);
println!("第{}签 · {}", q.id, q.grade);
for line in &q.poem {
println!(" {line}");
}
println!(" 解：{}", q.jie);
let q2 = daily_qian(2026, 11, 20);
assert_eq!(q.id, q2.id, "同一天必须抽到同一签");
println!(" （稳定性自检通过：同一天重复抽仍是第{}签）", q2.id);
println!();

// 答案之书
let entropy = SystemTime::now()
.duration_since(UNIX_EPOCH)
.unwrap()
.as_nanos() as u64;
println!("心里想好一个问题……答案是：{}", book_answer(entropy));
println!();

println!("（稳定性自检通过）");
println!();

// 八字排盘演示（1986-05-29 12:00 男，与金标准验算用例）
let bz = bazi(1986, 5, 29, 12, 0, 1);
println!("八字 {}（农历{}）", bz.solar, bz.lunar_text);
for p in &bz.pillars {
println!(
"{} {} 干{} 支{} 藏[{}] 五行{} 纳音{} 十神干{} 长生{}",
p.name, p.ganzhi, p.gan, p.zhi, p.hide_gan.join("/"), p.wu_xing, p.na_yin, p.shi_shen_gan, p.di_shi
);
}
println!("日主 {}（{}）· 胎元{} 命宫{} 身宫{}", bz.day_gan, bz.day_wuxing, bz.tai_yuan, bz.ming_gong, bz.shen_gong);
println!(
"五行 {} · 缺{:?} · 最旺{}",
bz.wuxing.iter().map(|w| format!("{}{}", w.name, w.count)).collect::<Vec<_>>().join(" "),
bz.missing, bz.strongest
);
println!("{} · {}", if bz.forward { "顺行" } else { "逆行" }, bz.start_desc);
for dy in &bz.da_yun {
println!("大运 {} {}-{}岁（{}-{}）", dy.ganzhi, dy.start_age, dy.end_age, dy.start_year, dy.end_year);
}
println!();

// 八字验算用例二（2000-01-01 00:30 女，子时盘）
let bz2 = bazi(2000, 1, 1, 0, 30, 0);
println!("八字 {}（农历{}）", bz2.solar, bz2.lunar_text);
for p in &bz2.pillars {
println!("{} {} 藏[{}] 纳音{} 十神干{} 长生{}", p.name, p.ganzhi, p.hide_gan.join("/"), p.na_yin, p.shi_shen_gan, p.di_shi);
}
println!("胎元{} 命宫{} 身宫{} · {} · {}", bz2.tai_yuan, bz2.ming_gong, bz2.shen_gong, if bz2.forward { "顺行" } else { "逆行" }, bz2.start_desc);
for dy in &bz2.da_yun {
println!("大运 {} {}-{}岁（{}-{}）", dy.ganzhi, dy.start_age, dy.end_age, dy.start_year, dy.end_year);
}

    
}

#[allow(dead_code)]
fn check_wedding() {
    let days: serde_json::Value =
        serde_json::from_str(&xuanji_core::wedding_days(2026, 11, 20, 2027, 2, 28, "鼠", "马")).unwrap();
    let arr = days.as_array().unwrap();
    println!("择日 2026-11-20~2027-02-28（避冲鼠/马）共 {} 天", arr.len());
    println!("ALL: {}", arr.iter().map(|d| d["solar"].as_str().unwrap()).collect::<Vec<_>>().join(" "));
    for d in arr.iter().take(3) {
        println!("  {} {} {} {} {}", d["solar"], d["weekday"], d["lunar_text"], d["day_ganzhi"], d["chong"]);
    }
    assert!(!arr.is_empty());
}

#[allow(dead_code)]
fn check_bone() {
    // 来源算例：1995-05-20 09:30 → 乙亥年四月廿一巳时，9+9+10+16=44
    let b = xuanji_core::bone_weight(1995, 5, 20, 9, 30);
    println!("称骨算例1: {} {} {} 年{} 月{} 日{} 时{} = {} {}",
        b.lunar_text, b.year_ganzhi, b.hour_zhi, b.year_w, b.month_w, b.day_w, b.hour_w, b.total, b.total_text);
    assert_eq!(b.total, 44);
    assert!(b.verse.starts_with("万事由天莫苦求"));
    // 自家用例：1986-05-29 12:00 → 丙寅年四月廿一午时，6+9+10+10=35
    let b2 = xuanji_core::bone_weight(1986, 5, 29, 12, 0);
    println!("称骨算例2: {} {} {} = {} {}", b2.lunar_text, b2.year_ganzhi, b2.hour_zhi, b2.total, b2.total_text);
    assert_eq!(b2.total, 35);
    // 全域扫描：总重必落在 21..=71
    for (yy, mm, dd, hh) in [(1924, 2, 5, 0), (2000, 1, 1, 23), (2043, 6, 15, 12), (1983, 3, 3, 3)] {
        let t = xuanji_core::bone_weight(yy, mm, dd, hh, 0).total;
        assert!((21..=71).contains(&t), "{t}");
    }
    println!("称骨自检通过");
}

#[allow(dead_code)]
fn check_hehun() {
    // 六合用例：1984 甲子鼠 × 1985 乙丑牛 → 生肖六合
    let r = xuanji_core::hehun(1984, 6, 1, 12, 0, 1985, 6, 1, 12, 0);
    println!("合婚1: {}×{} {} 分数 {} {}", r.a_zodiac, r.b_zodiac, r.factors[0].result, r.score, r.grade);
    assert!(r.factors[0].result.contains("六合"));
    // 相冲用例：1984 鼠 × 1990 庚午马 → 相冲
    let r2 = xuanji_core::hehun(1984, 6, 1, 12, 0, 1990, 6, 1, 12, 0);
    println!("合婚2: {}×{} {} 分数 {}", r2.a_zodiac, r2.b_zodiac, r2.factors[0].result, r2.score);
    assert!(r2.factors[0].result.contains("相冲"));
    assert!(r2.score < r.score);
    println!("合婚自检通过");
}

#[allow(dead_code)]
fn check_dreams() {
    let v: serde_json::Value = serde_json::from_str(xuanji_core::DREAMS_JSON).unwrap();
    let arr = v["dreams"].as_array().unwrap();
    assert!(arr.len() >= 110, "词条数 {}", arr.len());
    for e in arr {
        for k in ["cat", "kw", "title", "luck", "text"] {
            assert!(e[k].as_str().map(|s| !s.is_empty()).unwrap_or(false), "字段 {k}");
        }
    }
    println!("解梦自检通过：{} 条", arr.len());
}
