use lunar_rust::{
    eight_char::{da_yun::DaYunRefHelper, yun::YunRefHelper, EightCharRefHelper},
    jie_qi::JieQiRefHelper,
    lunar::LunarRefHelper,
    solar::{self, SolarRefHelper},
};
use serde::Serialize;

const QIAN_JSON: &str = include_str!("../../content/qian.json");
const ANSWERS_JSON: &str = include_str!("../../content/answers.json");

/// splitmix64：把日期这类规整的种子打散成均匀的伪随机数，
/// 保证「同一天同一个人问，结果稳定；相邻两天结果跳变」。
fn hash64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D049BB133111EB);
    x ^ (x >> 31)
}

fn date_seed(y: i64, m: i64, d: i64) -> u64 {
    ((y as u64) << 16) ^ ((m as u64) << 8) ^ (d as u64)
}

// ---------- 1. 今日黄历 ----------

#[derive(Serialize)]
pub struct Almanac {
    pub solar: String,
    pub lunar_text: String,
    pub year_ganzhi: String,
    pub month_ganzhi: String,
    pub day_ganzhi: String,
    pub shengxiao: String,
    pub xingzuo: String,
    pub yi: Vec<String>,
    pub ji: Vec<String>,
    pub chong: String,
    pub sha: String,
    pub xiu: String,
    pub xiu_luck: String,
    pub pengzu_gan: String,
    pub pengzu_zhi: String,
    pub cai: String,
    pub xi: String,
    pub fu: String,
    pub tianshen: String,
    pub tianshen_luck: String,
    pub prev_jieqi: String,
    pub next_jieqi: String,
}

pub fn almanac(y: i64, m: i64, d: i64) -> Almanac {
    let solar = solar::from_ymdhms(y, m, d, 12, 0, 0);
    let lunar = solar.get_lunar();
    Almanac {
        solar: format!("{y}-{m:02}-{d:02}"),
        lunar_text: format!(
            "{}年{}月{}",
            lunar.get_year_in_chinese(),
            lunar.get_month_in_chinese(),
            lunar.get_day_in_chinese()
        ),
        year_ganzhi: lunar.get_year_in_gan_zhi(),
        month_ganzhi: lunar.get_month_in_gan_zhi(),
        day_ganzhi: lunar.get_day_in_gan_zhi(),
        shengxiao: lunar.get_year_sheng_xiao(),
        xingzuo: solar.get_xing_zuo(),
        yi: lunar.get_day_yi(Some(1)),
        ji: lunar.get_day_ji(Some(1)),
        chong: lunar.get_day_chong_desc(),
        sha: lunar.get_day_sha(),
        xiu: lunar.get_xiu(),
        xiu_luck: lunar.get_xiu_luck(),
        pengzu_gan: lunar.get_peng_zu_gan(),
        pengzu_zhi: lunar.get_peng_zu_zhi(),
        cai: lunar.get_position_cai_desc(),
        xi: lunar.get_position_xi_desc(),
        fu: lunar.get_position_fu_desc(),
        tianshen: lunar.get_day_tian_shen(),
        tianshen_luck: lunar.get_day_tian_shen_luck(),
        prev_jieqi: lunar
            .get_prev_jie_qi(Some(true))
            .map(|j| j.get_name())
            .unwrap_or_default(),
        next_jieqi: lunar
            .get_next_jie_qi(Some(true))
            .map(|j| j.get_name())
            .unwrap_or_default(),
    }
}

// ---------- 2. 今日一签 ----------

#[derive(Serialize, serde::Deserialize, Clone)]
pub struct Qian {
    pub id: u32,
    pub grade: String,
    pub poem: Vec<String>,
    pub jie: String,
}

fn qian_bank() -> Vec<Qian> {
    serde_json::from_str(QIAN_JSON).expect("qian.json 格式错误")
}

/// 同一天抽到的签固定不变（日期做种子），过一天换一签。
pub fn daily_qian(y: i64, m: i64, d: i64) -> Qian {
    let bank = qian_bank();
    let idx = (hash64(date_seed(y, m, d)) % bank.len() as u64) as usize;
    bank[idx].clone()
}

// ---------- 3. 答案之书 ----------

fn answer_bank() -> Vec<String> {
    serde_json::from_str(ANSWERS_JSON).expect("answers.json 格式错误")
}

/// 每次发问给一个新答案，随机性由调用方传入的熵（entropy）决定：
/// 命令行版用系统时间，网页版用浏览器随机数。
pub fn book_answer(entropy: u64) -> String {
    let bank = answer_bank();
    let idx = (hash64(entropy ^ 0xA17E5) % bank.len() as u64) as usize;
    bank[idx].clone()
}

// ---------- 4. 星座运势 ----------

pub const ZODIAC_NAMES: [&str; 12] = [
    "白羊座", "金牛座", "双子座", "巨蟹座", "狮子座", "处女座", "天秤座", "天蝎座",
    "射手座", "摩羯座", "水瓶座", "双鱼座",
];

/// 按公历月日查星座，返回 0-11 下标。
pub fn zodiac_index(month: i64, day: i64) -> usize {
    // 每月「新星座」的起始日：如 1 月 20 日起是水瓶座，此前仍是摩羯座
    let bounds = [20, 19, 21, 20, 21, 22, 23, 23, 23, 24, 23, 22];
    let m = month as u64;
    if (day as u64) < bounds[(m - 1) as usize] {
        ((m + 8) % 12) as usize
    } else {
        ((m + 9) % 12) as usize
    }
}

const LUCKY_COLORS: [&str; 12] = [
    "石榴红", "杏子黄", "薄荷绿", "月光白", "宝蓝", "豆沙粉", "墨黑", "葡萄紫",
    "橘子橙", "青花蓝", "银灰", "樱花粉",
];

const GUIDE_HIGH: [&str; 6] = [
    "宜主动出击，想做的事今天就开口。",
    "贵人运旺，重要的话挑今天说。",
    "适合开启新计划，第一步迈出去就顺了。",
    "财运在线，但见好就收，别贪心。",
    "表达欲和感染力都强，适合展示自己。",
    "拖延很久的事，今天能一口气推进大半。",
];
const GUIDE_MID: [&str; 6] = [
    "宜稳扎稳打，把手头的事收尾。",
    "适合整理和复盘，旧物里有新发现。",
    "与人相处多听少说，误会自然消散。",
    "小步前进也是前进，别跟人比速度。",
    "适合陪家人朋友，关系运在升温。",
    "钱财宜守不宜攻，冲动消费先放一夜。",
];
const GUIDE_LOW: [&str; 6] = [
    "宜躺平蓄力，今天不适合硬碰硬。",
    "少做决定，多睡觉，醒来再说。",
    "出门注意细节，钥匙钱包多检查一遍。",
    "情绪容易上头，重要的话明天再说。",
    "适合独处，给自己留一段空白时间。",
    "旧问题可能翻出来，平常心应对就好。",
];

#[derive(Serialize)]
pub struct Fortune {
    pub sign: String,
    pub overall: u32,
    pub love: u32,
    pub career: u32,
    pub wealth: u32,
    pub lucky_color: String,
    pub lucky_number: u32,
    pub guide: String,
}

/// 同一个星座在同一天的运势固定（日期+星座做种子）。
pub fn zodiac_fortune(sign: usize, y: i64, m: i64, d: i64) -> Fortune {    let base = hash64(date_seed(y, m, d) ^ ((sign as u64) << 40));
    let score = |salt: u64| -> u32 { 55 + (hash64(base ^ salt) % 45) as u32 };
    let overall = score(1);
    let bank: &[&str] = if overall >= 85 {
        &GUIDE_HIGH
    } else if overall >= 70 {
        &GUIDE_MID
    } else {
        &GUIDE_LOW
    };
    Fortune {
        sign: ZODIAC_NAMES[sign].to_string(),
        overall,
        love: score(2),
        career: score(3),
        wealth: score(4),
        lucky_color: LUCKY_COLORS[(hash64(base ^ 5) % 12) as usize].to_string(),
        lucky_number: 1 + (hash64(base ^ 6) % 9) as u32,
        guide: bank[(hash64(base ^ 7) % bank.len() as u64) as usize].to_string(),
    }
}

// ---------- 5. 八字排盘 ----------

#[derive(Serialize)]
pub struct Pillar {
    pub name: String,
    pub ganzhi: String,
    pub gan: String,
    pub zhi: String,
    pub hide_gan: Vec<String>,
    pub wu_xing: String,
    pub na_yin: String,
    pub shi_shen_gan: String,
    pub shi_shen_zhi: Vec<String>,
    pub di_shi: String,
}

#[derive(Serialize)]
pub struct WxCount {
    pub name: String,
    pub count: u32,
}

#[derive(Serialize)]
pub struct DaYunItem {
    pub ganzhi: String,
    pub start_age: i64,
    pub end_age: i64,
    pub start_year: i64,
    pub end_year: i64,
}

#[derive(Serialize)]
pub struct Bazi {
    pub solar: String,
    pub lunar_text: String,
    pub pillars: Vec<Pillar>,
    pub day_gan: String,
    pub day_wuxing: String,
    pub wuxing: Vec<WxCount>,
    pub missing: Vec<String>,
    pub strongest: String,
    pub tai_yuan: String,
    pub ming_gong: String,
    pub shen_gong: String,
    pub forward: bool,
    pub start_desc: String,
    pub da_yun: Vec<DaYunItem>,
}

/// 命宫自算：lunar_rust 的 get_ming_gong 在 32 位目标（WebAssembly）上
/// 会因 usize 减法下溢而崩溃（64 位本地构建则被回绕掩盖）。
/// 公式与原库一致，改用 i64 运算：命宫支 = 26 −（月支序 + 时支序），再回绕；
/// 干支序号 = 月柱序号 −（月支序 − 命宫支序），取六十甲子。
fn ming_gong(month_ganzhi: &str, month_zhi: &str, time_zhi: &str) -> String {
    // 与原库一致的 1 起始月支表（首元为空串）：寅=1 … 子=11、丑=12
    const MONTH_ZHI: [&str; 13] = ["", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥", "子", "丑"];
    const GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
    const ZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
    let mzi = MONTH_ZHI.iter().position(|&z| z == month_zhi).unwrap_or(0) as i64;
    let tzi = MONTH_ZHI.iter().position(|&z| z == time_zhi).unwrap_or(0) as i64;
    let mut zhi_index = 26 - (mzi + tzi);
    if zhi_index > 12 {
        zhi_index -= 12;
    }
    let chars: Vec<char> = month_ganzhi.chars().collect();
    let gi = GAN.iter().position(|&g| g == chars[0].to_string().as_str()).unwrap_or(0) as i64;
    let zi = ZHI.iter().position(|&z| z == chars[1].to_string().as_str()).unwrap_or(0) as i64;
    let month_idx = (0..60).find(|&i| i % 10 == gi && i % 12 == zi).unwrap_or(0);
    let idx = ((month_idx - (mzi - zhi_index)) % 60 + 60) % 60;
    format!("{}{}", GAN[(idx % 10) as usize], ZHI[(idx % 12) as usize])
}

fn gan_wuxing(gan: &str) -> &'static str {    match gan {
        "甲" | "乙" => "木",
        "丙" | "丁" => "火",
        "戊" | "己" => "土",
        "庚" | "辛" => "金",
        _ => "水",
    }
}

/// 八字排盘。gender：1 男、0 女（影响大运顺逆与起运，与历法库约定一致）。
pub fn bazi(y: i64, m: i64, d: i64, hour: i64, minute: i64, gender: i64) -> Bazi {
    let solar = solar::from_ymdhms(y, m, d, hour, minute, 0);
    let lunar = solar.get_lunar();
    let ec = lunar.get_eight_char();
    let mk = |name: &str,
              ganzhi: String,
              gan: String,
              zhi: String,
              hide_gan: Vec<String>,
              wu_xing: String,
              na_yin: String,
              ssg: String,
              ssz: Vec<String>,
              di_shi: String|
     -> Pillar {
        Pillar {
            name: name.to_string(),
            ganzhi,
            gan,
            zhi,
            hide_gan,
            wu_xing,
            na_yin,
            shi_shen_gan: ssg,
            shi_shen_zhi: ssz,
            di_shi,
        }
    };
    let pillars = vec![
        mk("年柱", ec.get_year(), ec.get_year_gan(), ec.get_year_zhi(), ec.get_year_hide_gan(), ec.get_year_wu_xing(), ec.get_year_na_yin(), ec.get_year_shi_shen_gan(), ec.get_year_shi_shen_zhi(), ec.get_year_di_shi()),
        mk("月柱", ec.get_month(), ec.get_month_gan(), ec.get_month_zhi(), ec.get_month_hide_gan(), ec.get_month_wu_xing(), ec.get_month_na_yin(), ec.get_month_shi_shen_gan(), ec.get_month_shi_shen_zhi(), ec.get_month_di_shi()),
        mk("日柱", ec.get_day(), ec.get_day_gan(), ec.get_day_zhi(), ec.get_day_hide_gan(), ec.get_day_wu_xing(), ec.get_day_na_yin(), ec.get_day_shi_shen_gan(), ec.get_day_shi_shen_zhi(), ec.get_day_di_shi()),
        mk("时柱", ec.get_time(), ec.get_time_gan(), ec.get_time_zhi(), ec.get_time_hide_gan(), ec.get_time_wu_xing(), ec.get_time_na_yin(), ec.get_time_shi_shen_gan(), ec.get_time_shi_shen_zhi(), ec.get_time_di_shi()),
    ];
    // 五行计数：四天干 + 四地支的全部藏干，逐个计入
    let order = ["金", "木", "水", "火", "土"];
    let mut counts = [0u32; 5];
    let idx = |wx: &str| order.iter().position(|&o| o == wx).unwrap();
    for p in &pillars {
        counts[idx(gan_wuxing(&p.gan))] += 1;
        for hg in &p.hide_gan {
            counts[idx(gan_wuxing(hg))] += 1;
        }
    }
    let wuxing: Vec<WxCount> = order
        .iter()
        .enumerate()
        .map(|(i, n)| WxCount { name: n.to_string(), count: counts[i] })
        .collect();
    let missing: Vec<String> = wuxing.iter().filter(|w| w.count == 0).map(|w| w.name.clone()).collect();
    let strongest = wuxing
        .iter()
        .max_by_key(|w| w.count)
        .map(|w| w.name.clone())
        .unwrap_or_default();
    let day_gan = ec.get_day_gan();
    let tai_yuan = ec.get_tai_yuan();
    // 命宫自算（原库实现在 WebAssembly 上会崩溃，见 ming_gong 函数注释）；
    // 身宫的原库实现是 i64 安全的，继续用
    let ming_gong = ming_gong(&ec.get_month(), &lunar.get_month_zhi_exact(), &lunar.get_time_zhi());
    let shen_gong = ec.get_shen_gong();
    // 大运：get_yun 按值消费句柄，放最后调用
    let yun = ec.get_yun(gender, None);
    let forward = yun.is_forward();
    let start_desc = format!(
        "出生后 {} 年 {} 个月 {} 天起运",
        yun.get_start_year(),
        yun.get_start_month(),
        yun.get_start_day()
    );
    let da_yun: Vec<DaYunItem> = yun
        .get_da_yun(Some(10))
        .into_iter()
        .filter(|dy| !dy.get_gan_zhi().is_empty())
        .take(8)
        .map(|dy| DaYunItem {
            ganzhi: dy.get_gan_zhi(),
            start_age: dy.get_start_age(),
            // 注：lunar_rust 移植版 get_end_age() 有误（返回了结束年份），
            // 按金标准 lunar-javascript 的定义自行计算：起始年龄 + 9
            end_age: dy.get_start_age() + 9,
            start_year: dy.get_start_year(),
            end_year: dy.get_end_year(),
        })
        .collect();
    Bazi {
        solar: format!("{y}-{m:02}-{d:02} {hour:02}:{minute:02}"),
        lunar_text: format!(
            "{}年{}月{}",
            lunar.get_year_in_chinese(),
            lunar.get_month_in_chinese(),
            lunar.get_day_in_chinese()
        ),
        pillars,
        day_wuxing: gan_wuxing(&day_gan).to_string(),
        day_gan,
        wuxing,
        missing,
        strongest,
        tai_yuan,
        ming_gong,
        shen_gong,
        forward,
        start_desc,
        da_yun,
    }
}

/// 16 型人格题库与人格文案（文件：content/personality.json，构建时嵌入）
pub const PERSONALITY_JSON: &str = include_str!("../../content/personality.json");

/// 返回完整人格题库 JSON（题目 + 16 型文案）
pub fn personality_json() -> String {
    PERSONALITY_JSON.to_string()
}

/// 人格计分：answers 为 48 个字符的作答串（'a'/'b'，按题序）。
/// 返回 {"type":"INTJ","dims":[{"dim":"EI","left":"E","right":"I",
/// "left_count":..,"right_count":..,"left_pct":..,"right_pct":..}, ...]}；
/// 平局时取每维前一个极（E/S/T/J）。长度不对返回 {"error":...}。
pub fn personality_score(answers: &str) -> String {
    let bank: serde_json::Value = match serde_json::from_str(PERSONALITY_JSON) {
        Ok(v) => v,
        Err(e) => return format!("{{\"error\":\"题库解析失败：{e}\"}}"),
    };
    let qs = match bank["questions"].as_array() {
        Some(a) => a.clone(),
        None => return "{\"error\":\"题库格式错误\"}".to_string(),
    };
    let chosen: Vec<char> = answers.chars().collect();
    if chosen.len() != qs.len() {
        return format!("{{\"error\":\"作答长度应为 {}，实际 {}\"}}", qs.len(), chosen.len());
    }
    let dims_def: [(&str, char, char); 4] = [
        ("EI", 'E', 'I'),
        ("SN", 'S', 'N'),
        ("TF", 'T', 'F'),
        ("JP", 'J', 'P'),
    ];
    let mut counts: std::collections::HashMap<char, i64> = std::collections::HashMap::new();
    for (i, q) in qs.iter().enumerate() {
        let key = if chosen[i] == 'a' { "a" } else if chosen[i] == 'b' { "b" } else {
            return "{\"error\":\"作答只能包含 a 或 b\"}".to_string();
        };
        if let Some(p) = q[key]["p"].as_str().and_then(|s| s.chars().next()) {
            *counts.entry(p).or_insert(0) += 1;
        }
    }
    let mut type_code = String::new();
    let mut dims_out = Vec::new();
    for (dim, l, r) in dims_def {
        let lc = *counts.get(&l).unwrap_or(&0);
        let rc = *counts.get(&r).unwrap_or(&0);
        let total = (lc + rc).max(1);
        let lp = (lc * 100 + total / 2) / total;
        type_code.push(if lc >= rc { l } else { r });
        dims_out.push(serde_json::json!({
            "dim": dim, "left": l.to_string(), "right": r.to_string(),
            "left_count": lc, "right_count": rc,
            "left_pct": lp, "right_pct": 100 - lp
        }));
    }
    serde_json::json!({"type": type_code, "dims": dims_out}).to_string()
}
