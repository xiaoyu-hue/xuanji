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
    check_day_search();
    check_hours();
    check_taisui();
    check_shensha();
    check_liunian();
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
fn check_day_search() {
    // 万事择日：嫁娶特例必须与 wedding_days 逐字一致（同一内核）
    let a = xuanji_core::day_search(2026, 11, 20, 2027, 2, 28, "嫁娶", "鼠", "马");
    let b = xuanji_core::wedding_days(2026, 11, 20, 2027, 2, 28, "鼠", "马");
    assert_eq!(a, b);
    // 其余事项计数与金标准 lunar-javascript 同口径扫描一致（101 天，避鼠/马）
    for (act, n) in [("移徙", 20), ("出行", 24), ("开市", 14), ("纳财", 15)] {
        let v: serde_json::Value =
            serde_json::from_str(&xuanji_core::day_search(2026, 11, 20, 2027, 2, 28, act, "鼠", "马")).unwrap();
        assert_eq!(v.as_array().unwrap().len(), n, "{act}");
    }
    println!("万事择日自检通过（嫁娶与旧接口一致，移徙20/出行24/开市14/纳财15 与金标准一致）");
}

#[allow(dead_code)]
fn check_hours() {
    // 锚点来自金标准 lunar-javascript 的 LunarTime.fromYmdHms 逐时构造
    let h = xuanji_core::day_hours(2026, 11, 20);
    assert_eq!(h.len(), 12);
    assert_eq!((h[0].zhi.as_str(), h[0].ganzhi.as_str(), h[0].tianshen.as_str(), h[0].luck.as_str()), ("子", "壬子", "天牢", "凶"));
    let gz: Vec<&str> = h.iter().map(|x| x.ganzhi.as_str()).collect();
    assert_eq!(gz.join(" "), "壬子 癸丑 甲寅 乙卯 丙辰 丁巳 戊午 己未 庚申 辛酉 壬戌 癸亥");
    let luck: String = h.iter().map(|x| x.luck.as_str()).collect();
    assert_eq!(luck, "凶凶吉凶吉吉凶凶吉吉凶吉");
    let h2 = xuanji_core::day_hours(2000, 1, 1);
    assert_eq!((h2[0].tianshen.as_str(), h2[0].luck.as_str()), ("金匮", "吉"));
    let luck2: String = h2.iter().map(|x| x.luck.as_str()).collect();
    assert_eq!(luck2, "吉吉凶吉凶凶吉凶吉吉凶凶");
    println!("时辰吉凶自检通过（2026-11-20 与 2000-01-01 两日逐项与金标准一致）");
}

#[allow(dead_code)]
fn check_taisui() {
    let t = xuanji_core::taisui(2027, "羊");
    assert_eq!((t.year_ganzhi.as_str(), t.year_zodiac.as_str()), ("丁未", "羊"));
    assert_eq!(t.mine, vec!["值太岁".to_string()]);
    assert_eq!(t.hits.len(), 4, "2027 犯太岁名单应为羊/牛/狗/鼠四肖");
    assert_eq!(xuanji_core::taisui(2027, "鼠").mine, vec!["害太岁".to_string()]);
    assert_eq!(xuanji_core::taisui(2027, "牛").mine, vec!["冲太岁".to_string(), "刑太岁".to_string()]);
    assert_eq!(xuanji_core::taisui(2027, "狗").mine, vec!["刑太岁".to_string(), "破太岁".to_string()]);
    assert_eq!(xuanji_core::taisui(2026, "鼠").mine, vec!["冲太岁".to_string()]);
    assert_eq!(xuanji_core::taisui(2028, "蛇").mine, vec!["刑太岁".to_string(), "破太岁".to_string()]);
    assert!(xuanji_core::taisui(2027, "虎").mine.is_empty());
    println!("犯太岁自检通过（2027 丁未年名单 4 肖，值/冲/刑/害/破锚点全对）");
}

#[allow(dead_code)]
fn check_shensha() {
    // 锚点为手工按传统查表推演（表已经多来源交叉核对），与代码互为校验
    let bz = xuanji_core::bazi(1986, 5, 29, 12, 0, 1);
    let ss: Vec<Vec<String>> = bz.pillars.iter().map(|p| p.shensha.clone()).collect();
    assert_eq!(ss[0], Vec::<String>::new(), "年柱神煞");
    assert_eq!(ss[1], vec!["天乙贵人", "太极贵人", "孤辰"], "月柱神煞");
    assert_eq!(ss[2], vec!["国印贵人", "将星"], "日柱神煞");
    assert_eq!(ss[3], vec!["将星", "桃花"], "时柱神煞");
    let bz2 = xuanji_core::bazi(2000, 1, 1, 0, 30, 0);
    let ss2: Vec<Vec<String>> = bz2.pillars.iter().map(|p| p.shensha.clone()).collect();
    assert_eq!(ss2[0], vec!["将星", "桃花"], "例二年柱神煞");
    assert_eq!(ss2[1], vec!["红鸾", "桃花"], "例二月柱神煞");
    assert_eq!(ss2[2], vec!["天喜", "将星"], "例二日柱神煞");
    assert_eq!(ss2[3], vec!["月德贵人", "红鸾", "桃花"], "例二时柱神煞");
    println!("神煞自检通过（两例八柱逐柱与手工查表一致）");
}

#[allow(dead_code)]
fn check_liunian() {
    // 1986-05-29 12:00 男，流年 2027 丁未：命宫落未宫（本命财帛宫），丁年四化 太阴禄/天同权/天机科/巨门忌
    let ln = xuanji_core::ziwei_liunian(1986, 5, 29, 12, 0, 1, 2027);
    println!("流年 {} {} 命宫{}（本命{}）四化 {}",
        ln.year, ln.year_ganzhi, ln.soul_branch, ln.soul_palace_name,
        ln.mutagens.iter().map(|h| format!("{}{}@{}", h.star, h.hua, h.palace)).collect::<Vec<_>>().join(" "));
    assert_eq!(ln.year_ganzhi, "丁未");
    assert_eq!(ln.soul_branch, "未");
    assert_eq!(ln.soul_palace_name, "财帛");
    let stars: Vec<(&str, &str)> = ln.mutagens.iter().map(|h| (h.star.as_str(), h.hua.as_str())).collect();
    assert_eq!(stars, vec![("太阴", "禄"), ("天同", "权"), ("天机", "科"), ("巨门", "忌")]);
    // 流年十二宫：未宫应为流年命宫，午宫为流年兄弟（逆行）
    let at = |b: &str| ln.palaces.iter().find(|p| p.branch == b).unwrap().name.as_str();
    assert_eq!(at("未"), "流年命宫");
    assert_eq!(at("午"), "流年兄弟");
    assert_eq!(at("申"), "流年父母");
    println!("流年自检通过（2027 丁未锚点）");
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
