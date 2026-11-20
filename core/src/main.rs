use std::time::{SystemTime, UNIX_EPOCH};
use xuanji_core::{almanac, bazi, book_answer, daily_qian, zodiac_fortune, zodiac_index, ZODIAC_NAMES};

fn main() {
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

// 星座运势：自检星座换算与地基库一致，再输出今日天蝎运势
let idx = zodiac_index(11, 20);
assert!(ZODIAC_NAMES[idx].starts_with(&a.xingzuo), "星座换算须与地基一致");
let f = zodiac_fortune(idx, 2026, 11, 20);
println!(
"{} 今日：整体{} 爱情{} 事业{} 财运{}",
f.sign, f.overall, f.love, f.career, f.wealth
);
println!("幸运色 {} · 幸运数字 {} · {}", f.lucky_color, f.lucky_number, f.guide);
let f2 = zodiac_fortune(idx, 2026, 11, 20);
assert_eq!(f.overall, f2.overall, "同一星座同一天运势必须稳定");
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
check_personality();
}

#[allow(dead_code)]
fn check_personality() {
    let bank: serde_json::Value = serde_json::from_str(xuanji_core::PERSONALITY_JSON).unwrap();
    let qs = bank["questions"].as_array().unwrap();
    assert_eq!(qs.len(), 48, "题目总数应为 48");
    assert_eq!(bank["types"].as_object().unwrap().len(), 16, "人格类型应为 16");
    // 定向答卷：每题都选 INTJ 对应极，应得 INTJ 且四维 100%
    let target = ['I', 'N', 'T', 'J'];
    let dim_idx = |d: &str| match d { "EI" => 0, "SN" => 1, "TF" => 2, _ => 3 };
    let mut sheet = String::new();
    for q in qs {
        let want = target[dim_idx(q["dim"].as_str().unwrap())];
        sheet.push(if q["a"]["p"].as_str().unwrap() == want.to_string() { 'a' } else { 'b' });
    }
    let r: serde_json::Value = serde_json::from_str(&xuanji_core::personality_score(&sheet)).unwrap();
    assert_eq!(r["type"], "INTJ", "定向答卷应得 INTJ，实际：{r}");
    assert!(r["dims"].as_array().unwrap().iter().all(|d| d["left_pct"].as_i64().unwrap() == 0 || d["right_pct"].as_i64().unwrap() == 0),
        "定向答卷每维应为 100% 对 0%：{r}");
    println!("PERSONALITY CHECK OK 定向 INTJ 答卷 => INTJ（四维 100%）");
    let all_a: serde_json::Value = serde_json::from_str(&xuanji_core::personality_score(&"a".repeat(48))).unwrap();
    println!("ALL-A SHEET => {}", all_a["type"]);
}
