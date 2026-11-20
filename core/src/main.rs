use std::time::{SystemTime, UNIX_EPOCH};
use xuanji_core::{almanac, book_answer, daily_qian, zodiac_fortune, zodiac_index, ZODIAC_NAMES};

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
}
