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
    if std::env::var("NATAL_DUMP").is_ok() { natal_dump(); return; }
    check_wedding();
    check_day_search();
    check_hours();
    check_taisui();
    check_shensha();
    check_liunian();
    check_zhouyi();
    check_meihua();
    check_feixing();
    check_liuyao();
    check_qimen();
    check_west_mind();
    check_natal();
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
fn check_zhouyi() {
    // 手工锚点：全老阳→乾之坤六爻皆变带用九；全少阳→乾不变；全少阴→坤不变
    let a = xuanji_core::build_from_values([9; 6]);
    assert_eq!(a.ben.n, 1);
    assert_eq!(a.bian.as_ref().unwrap().n, 2);
    assert_eq!(a.yong.as_deref(), Some("见群龙无首，吉。"));
    assert_eq!(a.lines[0].name, "初九");
    assert_eq!(a.lines[5].name, "上九");
    let b = xuanji_core::build_from_values([7; 6]);
    assert_eq!(b.ben.n, 1);
    assert!(b.bian.is_none());
    let c = xuanji_core::build_from_values([8; 6]);
    assert_eq!(c.ben.n, 2);
    assert!(c.bian.is_none());
    // 初爻独变：乾→姤（44）；上爻独变：乾→夬（43，上卦成兑）；
    // 三爻独变：乾→履（10，下卦成兑）
    let d = xuanji_core::build_from_values([9, 7, 7, 7, 7, 7]);
    assert_eq!(d.bian.as_ref().unwrap().n, 44);
    assert_eq!(d.lines[0].text, "潜龙勿用。");
    let e = xuanji_core::build_from_values([7, 7, 7, 7, 7, 9]);
    assert_eq!(e.bian.as_ref().unwrap().n, 43);
    let g = xuanji_core::build_from_values([7, 7, 9, 7, 7, 7]);
    assert_eq!(g.bian.as_ref().unwrap().n, 10);
    // 种子通路：只验结构（六爻齐、点数 6..=9、本变卦自洽）
    let f = xuanji_core::zhouyi_draw(20261120);
    assert_eq!(f.lines.len(), 6);
    assert!(f.lines.iter().all(|l| (6..=9).contains(&l.value)));
    println!("周易自检通过（乾坤锚点＋独变锚点＋种子结构）");
}

#[allow(dead_code)]
fn check_meihua() {
    // 经典观梅占同构用例：甲辰年腊月十七申时（2025-01-16 16:00）
    // 年支辰5＋月12＋日17＝34，上卦兑；加时支申9＝43，下卦离，动爻1
    // 本卦泽火革（49），初爻变→泽山咸（31），体兑金、用离火→用克体
    let r = xuanji_core::meihua(2025, 1, 16, 16);
    println!("梅花 {} 本卦{} 动{} 变{} 体{}用{} {}",
        r.lunar_text, r.ben.full, r.dong_name, r.bian.full, r.ti_gua, r.yong_gua, r.relation);
    assert_eq!(r.ben.n, 49);
    assert_eq!(r.dong_yao, 1);
    assert_eq!(r.dong_name, "初九");
    assert_eq!(r.dong_text, "巩用黄牛之革。");
    assert_eq!(r.bian.n, 31);
    assert_eq!(r.ti_gua, "兑");
    assert_eq!(r.yong_gua, "离");
    assert_eq!(r.relation, "用克体");
    println!("梅花自检通过（观梅占同构锚点）");
}

#[allow(dead_code)]
fn check_feixing() {
    // 2025 乙巳年全盘（两处独立来源交叉核对）：二黑入中
    let f = xuanji_core::feixing(2025);
    assert_eq!(f.ganzhi, "乙巳");
    assert_eq!(f.center, 2);
    let got: Vec<u8> = f.palaces.iter().map(|p| p.star).collect();
    // 盘面行序 [巽 离 坤 / 震 中 兑 / 艮 坎 乾]
    assert_eq!(got, vec![1, 6, 8, 9, 2, 4, 5, 7, 3]);
    assert_eq!(xuanji_core::feixing(2024).center, 3);
    assert_eq!(xuanji_core::feixing(2026).center, 1);
    assert_eq!(xuanji_core::feixing(2027).center, 9);
    println!("飞星自检通过（2025 全盘九宫与公开盘一致）");
}

#[allow(dead_code)]
fn check_liuyao() {
    // 乾为天静卦，2025-02-24 甲子日（月建寅、旬空戌亥）
    let r = xuanji_core::liuyao_from_values([7; 6], 2025, 2, 24);
    assert_eq!(r.ben.n, 1);
    assert_eq!(r.palace, "乾");
    assert_eq!(r.shi, 6);
    assert_eq!(r.ying, 3);
    assert_eq!(r.day_ganzhi, "甲子");
    assert_eq!(r.yue_jian, "寅");
    assert_eq!(r.kong_wang, ["戌", "亥"]);
    let zhis: Vec<&str> = r.lines.iter().map(|l| l.zhi.as_str()).collect();
    assert_eq!(zhis, ["子", "寅", "辰", "午", "申", "戌"]);
    let qs: Vec<&str> = r.lines.iter().map(|l| l.liuqin.as_str()).collect();
    assert_eq!(qs, ["子孙", "妻财", "父母", "官鬼", "兄弟", "父母"]);
    let ss: Vec<&str> = r.lines.iter().map(|l| l.shen.as_str()).collect();
    assert_eq!(ss, ["青龙", "朱雀", "勾陈", "螣蛇", "白虎", "玄武"]);
    assert!(r.lines[5].kong && !r.lines[0].kong);
    assert!(r.lines[5].is_shi && r.lines[2].is_ying);
    // 乾初爻变→姤：变卦初爻丑土父母（按乾宫论六亲）
    let v = xuanji_core::liuyao_from_values([9, 7, 7, 7, 7, 7], 2025, 2, 24);
    assert_eq!(v.bian.as_ref().unwrap().n, 44);
    let bl = v.bian_lines.as_ref().unwrap();
    assert_eq!(bl[0].zhi, "丑");
    assert_eq!(bl[0].liuqin, "父母");
    assert!(bl[0].changing && !bl[5].changing);
    // 八宫归属抽查（生成规则自检）：晋为乾宫游魂世四、同人为离宫归魂世三
    let jin = xuanji_core::liuyao_from_values([8, 8, 8, 7, 8, 7], 2025, 2, 24);
    assert_eq!(jin.ben.n, 35);
    assert_eq!((jin.palace.as_str(), jin.shi), ("乾", 4));
    let tong = xuanji_core::liuyao_from_values([7, 8, 7, 7, 7, 7], 2025, 2, 24);
    assert_eq!(tong.ben.n, 13);
    assert_eq!((tong.palace.as_str(), tong.shi), ("离", 3));
    println!("六爻自检通过（乾卦装卦全项＋姤变卦＋八宫生成）");
}

mod qimen_gold {
    include!("../../goldcheck/qimen_fixture.rs");
}

#[allow(dead_code)]
fn check_qimen() {
    // 与 qimen-rs（MIT）八个时刻全盘逐宫对拍：盘头 + 九宫六项全等才算过
    for g in qimen_gold::GOLD {
        let c = xuanji_core::qimen_chart(g.y, g.m, g.d, g.h);
        let head = format!("{}-{}-{} {}时", g.y, g.m, g.d, g.h);
        assert_eq!(c.dun, g.dun, "{head} 遁");
        assert_eq!(c.ju, g.ju, "{head} 局");
        assert_eq!(c.yuan, g.yuan, "{head} 元");
        assert_eq!(c.day_ganzhi, g.day, "{head} 日柱");
        assert_eq!(c.hour_ganzhi, g.hour, "{head} 时柱");
        assert_eq!(c.zhifu_star, g.zhifu, "{head} 值符");
        assert_eq!(c.zhifu_palace, g.zhifu_pal, "{head} 值符宫");
        assert_eq!(c.zhishi_door, g.zhishi, "{head} 值使");
        assert_eq!(c.zhishi_palace, g.zhishi_pal, "{head} 值使宫");
        assert_eq!(c.kong_wang, g.kong, "{head} 旬空");
        assert_eq!(c.horse_zhi, g.horse_zhi, "{head} 驿马");
        assert_eq!(c.horse_palace, g.horse_pal, "{head} 驿马宫");
        for cell in &c.palaces {
            let e = &g.palaces[(cell.num - 1) as usize];
            let mut heaven = cell.heaven.join(",");
            if let Some(h) = &cell.heaven_hosted {
                if !heaven.is_empty() {
                    heaven.push(',');
                }
                heaven.push_str(&format!("{h}寄"));
            }
            assert_eq!(cell.earth, e.0, "{head} {}宫地盘", cell.num);
            assert_eq!(heaven, e.1, "{head} {}宫天盘", cell.num);
            assert_eq!(cell.star, e.2, "{head} {}宫九星", cell.num);
            assert_eq!(cell.door.as_deref().unwrap_or(""), e.3, "{head} {}宫八门", cell.num);
            assert_eq!(cell.god.as_deref().unwrap_or(""), e.4, "{head} {}宫八神", cell.num);
            assert_eq!(cell.hosted_earth.as_deref().unwrap_or(""), e.5, "{head} {}宫寄地盘", cell.num);
        }
    }
    println!("奇门自检通过（与 qimen-rs 八盘逐宫全对拍一致）");
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

/// 5.5.0 西方馆＋心智馆自检：恢复三件回归锚点＋新做四件结构/计分锚点。
fn check_west_mind() {
    use xuanji_core::*;
    // 星座：分界锚点（1/19 摩羯 9、1/20 水瓶 10、11/22 前天蝎 7）
    assert_eq!(zodiac_index(1, 19), 9);
    assert_eq!(zodiac_index(1, 20), 10);
    assert_eq!(zodiac_index(11, 22), 7);
    let f = zodiac_fortune(0, 2026, 11, 20);
    assert_eq!(f.sign, "白羊座");
    assert!((55..=99).contains(&f.overall));
    // 塔罗：同日固定、牌库 22
    assert_eq!(daily_tarot(2026, 11, 20), daily_tarot(2026, 11, 20));
    let tv: serde_json::Value = serde_json::from_str(&daily_tarot(2026, 11, 20)).unwrap();
    assert!(tv["num"].as_u64().unwrap() <= 21);
    // 人格：全选 a → ISTJ（2.1.0 公网验算锚点）
    let ps: serde_json::Value = serde_json::from_str(&personality_score(&"a".repeat(48))).unwrap();
    assert_eq!(ps["type"], "ISTJ", "全 a 应为 ISTJ，实际 {}", ps);
    // 生命灵数：1990-01-01 → 路径 3、生日 1、态度 2；2026 个人年 3
    let lp = lifepath(1990, 1, 1, 2026);
    assert_eq!(lp.life_path.n, 3);
    assert_eq!(lp.birthday.n, 1);
    assert_eq!(lp.attitude.n, 2);
    assert_eq!(lp.personal_year.n, 3);
    // 大师数：1999-09-01 数字和 28+9+1=38→11 停步
    let lp2 = lifepath(1999, 9, 1, 2026);
    assert_eq!(lp2.life_path.n, 11, "1999-09-01 应为大师数 11，实际 {}", lp2.life_path.n);
    // 卢恩：24 枚齐全、同日固定、三卢恩互不重复
    assert_eq!(runes::RUNES.len(), 24);
    assert_eq!(daily_rune(2026, 11, 20).name, daily_rune(2026, 11, 20).name);
    let d3 = runes_draw3(42);
    assert_eq!(d3.len(), 3);
    assert!(d3[0].name != d3[1].name && d3[1].name != d3[2].name && d3[0].name != d3[2].name);
    // 大五：全 3 → 五维皆 30；全 5 → E30 A34 C34 S18 O38（反向键锚点）
    let mid: serde_json::Value = serde_json::from_str(&bigfive_score(&"3".repeat(50))).unwrap();
    for t in mid["traits"].as_array().unwrap() {
        assert_eq!(t["score"], 30, "全 3 时 {} 应为 30", t["key"]);
    }
    let hi: serde_json::Value = serde_json::from_str(&bigfive_score(&"5".repeat(50))).unwrap();
    let expect = [("E", 30), ("A", 34), ("C", 34), ("S", 18), ("O", 38)];
    for (k, v) in expect {
        let got = hi["traits"].as_array().unwrap().iter().find(|t| t["key"] == k).unwrap()["score"].as_u64().unwrap();
        assert_eq!(got, v, "全 5 时 {k} 应为 {v}，实际 {got}");
    }
    // 霍兰德：R 题全 5 其余 1 → 代码以 R 开头、R=40
    let mut ans = vec!['1'; 48];
    for c in ans.iter_mut().take(8) { *c = '5'; }
    let hs: serde_json::Value = serde_json::from_str(&holland_score(&ans.iter().collect::<String>())).unwrap();
    assert!(hs["code"].as_str().unwrap().starts_with('R'));
    assert_eq!(hs["types"][0]["score"], 40);
    println!("西方馆＋心智馆自检通过（星座分界/塔罗同日/人格 ISTJ/灵数大师数/卢恩 24/大五反向键/霍兰德 R 锚点）");
}

/// 星盘对拍导出：与 pyswisseph 金标准逐行星比对（NATAL_DUMP=1 时启用）
fn natal_dump() {
    use xuanji_core::natal_chart;
    let cases = [
        (1990, 1, 1, 12, 0, 116.407, 39.904, "A"),
        (1986, 5, 29, 12, 0, 116.407, 39.904, "B"),
        (2000, 6, 15, 8, 30, 121.473, 31.230, "C"),
        (2024, 2, 10, 0, 30, 113.264, 23.129, "D"),
        (1975, 11, 20, 23, 45, 87.617, 43.792, "E"),
    ];
    for (y, m, d, hh, mm, lon, lat, tag) in cases {
        let c = natal_chart(y, m, d, hh, mm, lon, lat);
        let lons: Vec<String> = c.planets.iter().map(|p| format!("{:.3}", p.lon)).collect();
        println!("{tag} planets {} asc {:.3} mc {:.3}", lons.join(" "), c.asc.lon, c.mc.lon);
    }
}

/// 6.1.0 星盘自检：与瑞士星历（pyswisseph）金标准对拍的锚点
/// （五盘 60 个数值全对拍，最大差 0.137°，见 goldcheck/natal_gold.py）。
fn check_natal() {
    use xuanji_core::natal_chart;
    // A 盘 1990-01-01 12:00 北京：金标准黄经（瑞士星历）
    let gold_a = [280.474, 328.789, 295.760, 306.263, 249.766, 95.194, 285.618, 275.765, 282.025, 227.084];
    let a = natal_chart(1990, 1, 1, 12, 0, 116.407, 39.904);
    for (p, g) in a.planets.iter().zip(gold_a.iter()) {
        let d = (p.lon - g).abs();
        assert!(d < 0.2, "A 盘 {} 黄经差 {d:.3f}", p.name);
    }
    assert!((a.asc.lon - 11.837).abs() < 0.1, "A 盘上升 {}", a.asc.lon);
    assert!((a.mc.lon - 276.389).abs() < 0.1, "A 盘中天 {}", a.mc.lon);
    assert_eq!(a.planets[0].sign, "摩羯座");
    assert_eq!(a.asc.sign, "白羊座");
    // C 盘 2000-06-15 08:30 上海：土星落座金牛（金标准 54.846）
    let c = natal_chart(2000, 6, 15, 8, 30, 121.473, 31.230);
    assert_eq!(c.planets[6].sign, "金牛座", "C 盘土星 {}", c.planets[6].lon);
    assert!((c.asc.lon - 131.101).abs() < 0.1, "C 盘上升 {}", c.asc.lon);
    // 宫位自洽：太阳宫位在 1..=12、宫头自上升起每 30° 一宫
    assert!((1..=12).contains(&a.planets[0].house));
    assert!((a.houses[1] - a.houses[0] - 30.0).abs() < 0.01);
    println!("星盘自检通过（与瑞士星历五盘对拍：行星/上升/中天锚点一致）");
}
