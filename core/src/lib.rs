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

// ---------- 4. 八字排盘 ----------

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

// ---------- 5. 婚嫁择日 ----------

#[derive(Serialize)]
pub struct WeddingDay {
    pub solar: String,
    pub weekday: String,
    pub lunar_text: String,
    pub day_ganzhi: String,
    pub chong: String,
}

/// 公历日期转「自 1970-01-01 起的天数」（Howard Hinnant 的 civil 公式），
/// 用于星期计算与日期递增，避免引入日期库。
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y_adj = if m <= 2 { y - 1 } else { y };
    let era = if y_adj >= 0 { y_adj } else { y_adj - 399 } / 400;
    let yoe = y_adj - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// 婚嫁择日：在 [起, 止] 公历区间内筛「宜嫁娶」且不冲新郎、新娘生肖的日子。
/// groom / bride 为生肖单字（鼠牛虎兔龙蛇马羊猴鸡狗猪），空串表示不限。
/// 返回 JSON 数组（最多 120 条，区间最长 400 天，超限自动截断）。
pub fn wedding_days(
    y1: i64, m1: i64, d1: i64,
    y2: i64, m2: i64, d2: i64,
    groom: &str, bride: &str,
) -> String {
    let start = days_from_civil(y1, m1, d1);
    let mut end = days_from_civil(y2, m2, d2);
    if end < start {
        return "[]".to_string();
    }
    if end - start > 400 {
        end = start + 400;
    }
    const WEEK: [&str; 7] = ["日", "一", "二", "三", "四", "五", "六"];
    let mut out: Vec<WeddingDay> = Vec::new();
    let mut z = start;
    while z <= end && out.len() < 120 {
        let (y, m, d) = civil_from_days(z);
        let solar = solar::from_ymdhms(y, m, d, 12, 0, 0);
        let lunar = solar.get_lunar();
        let yi = lunar.get_day_yi(Some(1));
        let ji = lunar.get_day_ji(Some(1));
        if yi.iter().any(|x| x == "嫁娶") && !ji.iter().any(|x| x == "嫁娶") {
            let chong_sx = lunar.get_day_chong_sheng_xiao();
            if (groom.is_empty() || chong_sx != groom) && (bride.is_empty() || chong_sx != bride) {
                // 1970-01-01 是星期四，WEEK 下标 4
                let wd = WEEK[(((z + 4) % 7 + 7) % 7) as usize];
                out.push(WeddingDay {
                    solar: format!("{y}-{m:02}-{d:02}"),
                    weekday: format!("星期{wd}"),
                    lunar_text: format!(
                        "{}年{}月{}",
                        lunar.get_year_in_chinese(),
                        lunar.get_month_in_chinese(),
                        lunar.get_day_in_chinese()
                    ),
                    day_ganzhi: lunar.get_day_in_gan_zhi(),
                    chong: lunar.get_day_chong_desc(),
                });
            }
        }
        z += 1;
    }
    serde_json::to_string(&out).unwrap()
}

// ---------- 6. 称骨（袁天罡称骨歌，民间通行本） ----------
// 骨重表经三处独立来源交叉核对（见 docs/开发文档.md）；癸亥年取七钱
// （开源实现多数派，另有六钱一说）。年按农历年干支（春节为界），
// 闰月按本月，时辰按钟点取。

/// 年骨：六十甲子序（甲子=0），单位钱
const YEAR_W: [i64; 60] = [
    12, 9, 6, 7, 12, 5, 9, 8, 7, 8,
    15, 9, 16, 8, 8, 19, 12, 6, 8, 7,
    5, 15, 6, 16, 15, 7, 9, 12, 10, 7,
    15, 6, 5, 14, 14, 9, 7, 7, 9, 12,
    8, 7, 13, 5, 14, 5, 9, 17, 5, 7,
    12, 8, 8, 6, 19, 6, 8, 16, 10, 7,
];
/// 月骨：农历正月至腊月，单位钱
const MONTH_W: [i64; 12] = [6, 7, 18, 9, 5, 16, 9, 15, 18, 8, 9, 5];
/// 日骨：农历初一至三十，单位钱
const DAY_W: [i64; 30] = [
    5, 10, 8, 15, 16, 15, 8, 16, 8, 16,
    9, 17, 8, 17, 10, 8, 9, 18, 5, 15,
    10, 9, 8, 9, 15, 18, 7, 8, 16, 6,
];
/// 时骨：子至亥，单位钱
const HOUR_W: [i64; 12] = [16, 6, 7, 10, 9, 16, 10, 8, 8, 9, 6, 6];

/// 称骨歌：下标 = 总骨重（钱）− 21，即二两一钱至七两一钱共 51 首，
/// 末首七两二钱为旧本增补（按表无人能称到，附此存目）。
const VERSES: [&str; 52] = [
    "短命非业谓大空，平生灾难事重重；凶祸频临陷逆境，终世困苦事不成。",
    "身寒骨冷苦伶仃，此命推来行乞人；劳劳碌碌难度日，终年打拱过平生。",
    "此命推来骨格轻，求谋作事事难成；妻儿子女应难许，别处他乡作散人。",
    "此命推来福禄无，门庭困苦总难荣；六亲骨肉皆无靠，流浪他乡作老翁。",
    "此命推来祖业微，门庭营度似希奇；六亲骨肉如冰炭，一世勤劳自把持。",
    "平生衣禄苦中求，独自营谋事不休；离祖出门宜早计，晚来衣食自无忧。",
    "一生作事少商量，难靠祖宗作主张；独马单枪空做去，早年晚岁总无长。",
    "一生行事似飘蓬，祖宗产业在梦中；若不过房改名姓，也当移徙二三通。",
    "初年运限未能通，纵有功名在后成；须过四旬才可立，移居改姓始为良。",
    "劳劳碌碌苦中求，东奔西走何日休；若能终身勤与俭，老来稍可免忧愁。",
    "忙忙碌碌苦中求，何日云开见日头；难得祖基家可立，中年衣食渐无忧。",
    "初年运蹇事难谋，渐有财源如水流；到得中年衣食旺，那时名利一齐收。",
    "早年作事事难成，百计勤劳枉费心；半世自如流水去，后来运到始得金。",
    "此命福气果如何，僧道门中衣禄多；离祖出家方为妙，朝晚拜佛念弥陀。",
    "生平福量不周全，祖业根基觉少传；营事生涯宜守旧，时来衣食胜从前。",
    "不须劳碌过平生，独自成家福不轻；早有福星常照命，任君行去百般成。",
    "此命般般事不成，弟兄少力自孤行；纵然祖业虽微有，来得明时去不明。",
    "一生骨肉最清高，早入黉门姓氏标；待到年将三十六，蓝衣脱去换红袍。",
    "此命终身运不通，劳劳作事尽皆空；苦心竭力成家计，到得那时在梦中。",
    "平生衣禄是绵长，件件心中自主张；前面风霜多受过，后来必定享安康。",
    "此命推来事不同，为人能干异凡庸；中年还有逍遥福，不比前时运未通。",
    "得宽怀处且宽怀，何用双眉皱不开；若使中年命运济，那时名利一齐来。",
    "为人心性最聪明，作事轩昂近贵人；衣禄一生天数定，不须劳碌是丰亨。",
    "万事由天莫苦求，须知福禄赖人修；当年财帛难如意，晚景欣然便不忧。",
    "名利推来竟若何，前番辛苦后奔波；命中难养男与女，骨肉扶持也不多。",
    "东西南北尽皆通，出姓移居更觉隆；衣禄无亏天数定，中年晚景一般同。",
    "此命推来旺末年，妻荣子贵自怡然；平生原有滔滔福，可有财源若水泉。",
    "幼年运道未曾亨，若是蹉跎命不同；兄弟六亲皆无靠，一身事业晚来隆。",
    "此命推来福不轻，自成自立显门庭；从来富贵人亲近，使婢差奴过一生。",
    "为利为名终日劳，中年福禄也多遭；老年自有财星照，不比前番目下高。",
    "一世荣华事事通，不须劳碌自亨通；兄弟叔侄皆如意，家业成时福禄宏。",
    "一世亨通事事能，不须劳苦自然宁；宗族欣然福禄多，家业丰亨自称心。",
    "此格推来福禄宏，兴家发达在其中；一生衣食安排定，却是人间一富翁。",
    "此命推来厚且清，诗书满腹看功成；丰衣足食自然稳，正是人间有福人。",
    "走马扬鞭争利名，少年作事费评论；一朝福禄源源至，富贵荣华显六亲。",
    "此格推来礼义通，一生福禄用无穷；甜酸苦辣皆尝过，财源滚滚稳且通。",
    "福禄丰盈万事全，一生荣耀显双亲；名扬威振人钦敬，处世逍遥似遇春。",
    "平生福禄自然来，名利兼全福寿偕；雁塔题名为贵客，紫袍玉带走金阶。",
    "细推此格秀且清，必定财高学业成；甲第之中应有分，扬鞭走马显威荣。",
    "一朝金榜快题名，显祖荣宗大器成；衣食自然丰裕足，田园财帛更丰盈。",
    "不作朝中金榜客，定为世上一财翁；聪明天赋经书熟，名显高科自是荣。",
    "此命生来福不穷，读书必定显亲族；紫衣金带为卿相，富贵荣华皆可同。",
    "命主为官福禄长，得来富贵实非常；名题雁塔传金榜，大显门庭天下扬。",
    "此命威权不可当，紫袍金带坐高堂；荣华富贵谁能及，百世留名姓氏扬。",
    "细推此命福不轻，富贵荣华孰与争；定国安邦人极品，威声显赫四方闻。",
    "此格人间一福人，堆金积玉满堂春；从来富贵由天定，金榜题名谒圣君。",
    "此命生来福自宏，田园家业最高隆；平生衣食丰盈足，一世荣华万事通。",
    "富贵由天莫苦求，万金家计不须谋；十年不比前番事，祖业根基水上舟。",
    "君是人间衣禄星，一生富贵众人钦；纵然福禄由天定，安享荣华过一生。",
    "此命推来福非轻，不须愁虑苦劳心；一生天定衣与禄，富贵荣华主一生。",
    "此命生成大不同，公侯卿相在其中；一生自有逍遥福，富贵荣华极品隆。",
    "此格世界罕有生，十代积善产此人；天上紫微来照命，统治万民乐太平。",
];

#[derive(Serialize)]
pub struct BoneWeight {
    pub lunar_text: String,
    pub year_ganzhi: String,
    pub hour_zhi: String,
    pub year_w: i64,
    pub month_w: i64,
    pub day_w: i64,
    pub hour_w: i64,
    pub total: i64,
    pub total_text: String,
    pub verse: String,
}

const CN_NUM: [&str; 10] = ["", "一", "二", "三", "四", "五", "六", "七", "八", "九"];

/// 称骨：公历出生年月日时分 → 四项骨重、总重与称骨歌。
pub fn bone_weight(y: i64, m: i64, d: i64, hour: i64, minute: i64) -> BoneWeight {
    let solar = solar::from_ymdhms(y, m, d, hour, minute, 0);
    let lunar = solar.get_lunar();
    let gz = lunar.get_year_in_gan_zhi();
    // 六十甲子序：由干、支下标反推（干 i%10、支 i%12，CRT 在 0..60 内唯一）
    const GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
    const ZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
    let g = gz.chars().next().unwrap().to_string();
    let z = gz.chars().nth(1).unwrap().to_string();
    let gi = GAN.iter().position(|x| *x == g).unwrap() as i64;
    let zi = ZHI.iter().position(|x| *x == z).unwrap() as i64;
    let mut jiazi = 0i64;
    for i in 0..60 {
        if i % 10 == gi && i % 12 == zi {
            jiazi = i;
            break;
        }
    }
    let year_w = YEAR_W[jiazi as usize];
    let lm = lunar.get_month().abs();
    let month_w = MONTH_W[(lm - 1) as usize];
    let day_w = DAY_W[(lunar.get_day() - 1) as usize];
    let hz = (((hour + 1) / 2) % 12) as usize;
    let hour_w = HOUR_W[hz];
    let total = year_w + month_w + day_w + hour_w;
    let liang = total / 10;
    let qian = total % 10;
    let total_text = if qian == 0 {
        format!("{}两整", CN_NUM[liang as usize])
    } else {
        format!("{}两{}钱", CN_NUM[liang as usize], CN_NUM[qian as usize])
    };
    BoneWeight {
        lunar_text: format!(
            "{}年{}月{}",
            lunar.get_year_in_chinese(),
            lunar.get_month_in_chinese(),
            lunar.get_day_in_chinese()
        ),
        year_ganzhi: gz,
        hour_zhi: format!("{}时", ZHI[hz]),
        year_w,
        month_w,
        day_w,
        hour_w,
        total,
        total_text,
        verse: VERSES[(total - 21) as usize].to_string(),
    }
}

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

// ---------- 8. 解梦（本站原创词条，content/dreams.json） ----------

pub const DREAMS_JSON: &str = include_str!("../../content/dreams.json");

pub fn dreams_json() -> String {
    DREAMS_JSON.to_string()
}

// ---------- 9. 紫微斗数 ----------
// 安星口径与 iztro（MIT）默认配置对齐并逐盘验算：年界春节、闰月十五后
// 按下月起命宫、晚子时（23 点）换日。星耀亮度为传统庙陷表。

const ZW_GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
const ZW_ZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
/// 宫名（自命宫逆行）
const PALACE_NAMES: [&str; 12] = [
    "命宫", "兄弟", "夫妻", "子女", "财帛", "疾厄", "迁移", "交友", "官禄", "田宅", "福德", "父母",
];

fn pmod(a: i64, m: i64) -> i64 {
    ((a % m) + m) % m
}

/// 星耀亮度表（庙旺得利平不陷），按宫位索引（寅=0）排列；空串为无亮度
fn brightness(star: &str, p: i64) -> &'static str {
    let t: &[&str; 12] = match star {
        "紫微" => &["旺", "旺", "得", "旺", "庙", "庙", "旺", "旺", "得", "旺", "平", "庙"],
        "天机" => &["得", "旺", "利", "平", "庙", "陷", "得", "旺", "利", "平", "庙", "陷"],
        "太阳" => &["旺", "庙", "旺", "旺", "旺", "得", "得", "平", "不", "陷", "陷", "不"],
        "武曲" => &["得", "利", "庙", "平", "旺", "庙", "得", "利", "庙", "平", "旺", "庙"],
        "天同" => &["利", "平", "平", "庙", "陷", "不", "旺", "平", "平", "庙", "旺", "不"],
        "廉贞" => &["庙", "平", "利", "陷", "平", "利", "庙", "平", "利", "陷", "平", "利"],
        "天府" => &["庙", "得", "庙", "得", "旺", "庙", "得", "旺", "庙", "得", "庙", "庙"],
        "太阴" => &["旺", "陷", "陷", "陷", "不", "不", "利", "旺", "旺", "庙", "庙", "庙"],
        "贪狼" => &["平", "利", "庙", "陷", "旺", "庙", "平", "利", "庙", "陷", "旺", "庙"],
        "巨门" => &["庙", "庙", "陷", "旺", "旺", "不", "庙", "庙", "陷", "旺", "旺", "不"],
        "天相" => &["庙", "陷", "得", "得", "庙", "得", "庙", "陷", "得", "得", "庙", "庙"],
        "天梁" => &["庙", "庙", "庙", "陷", "庙", "旺", "陷", "得", "庙", "陷", "庙", "旺"],
        "七杀" => &["庙", "旺", "庙", "平", "旺", "庙", "庙", "旺", "庙", "平", "旺", "庙"],
        "破军" => &["得", "陷", "旺", "平", "庙", "旺", "得", "陷", "旺", "平", "庙", "旺"],
        "文昌" => &["陷", "利", "得", "庙", "陷", "利", "得", "庙", "陷", "利", "得", "庙"],
        "文曲" => &["平", "旺", "得", "庙", "陷", "旺", "得", "庙", "陷", "旺", "得", "庙"],
        "火星" => &["庙", "利", "陷", "得", "庙", "利", "陷", "得", "庙", "利", "陷", "得"],
        "铃星" => &["庙", "利", "陷", "得", "庙", "利", "陷", "得", "庙", "利", "陷", "得"],
        "擎羊" => &["", "陷", "庙", "", "陷", "庙", "", "陷", "庙", "", "陷", "庙"],
        "陀罗" => &["陷", "", "庙", "陷", "", "庙", "陷", "", "庙", "陷", "", "庙"],
        _ => return "",
    };
    t[p as usize]
}

/// 四化表（按年干）：[化禄， 化权， 化科， 化忌]
fn mutagens(year_gan: &str) -> [&'static str; 4] {
    match year_gan {
        "甲" => ["廉贞", "破军", "武曲", "太阳"],
        "乙" => ["天机", "天梁", "紫微", "太阴"],
        "丙" => ["天同", "天机", "文昌", "廉贞"],
        "丁" => ["太阴", "天同", "天机", "巨门"],
        "戊" => ["贪狼", "太阴", "右弼", "天机"],
        "己" => ["武曲", "贪狼", "天梁", "文曲"],
        "庚" => ["太阳", "武曲", "太阴", "天同"],
        "辛" => ["巨门", "太阳", "文曲", "文昌"],
        "壬" => ["天梁", "紫微", "左辅", "武曲"],
        _ => ["破军", "巨门", "太阴", "贪狼"], // 癸
    }
}

#[derive(Serialize, Clone)]
pub struct ZwStar {
    pub name: String,
    pub kind: String,
    pub brightness: String,
    pub hua: String,
}

#[derive(Serialize)]
pub struct ZwPalace {
    pub branch: String,
    pub ganzhi: String,
    pub name: String,
    pub is_soul: bool,
    pub is_body: bool,
    pub daxian: String,
    pub stars: Vec<ZwStar>,
}

#[derive(Serialize)]
pub struct ZiweiChart {
    pub lunar_text: String,
    pub year_ganzhi: String,
    pub soul_ganzhi: String,
    pub body_ganzhi: String,
    pub ju: String,
    pub ziwei_branch: String,
    pub palaces: Vec<ZwPalace>,
}

/// 紫微排盘：公历出生年月日时分 + 性别（1 男 0 女）。
pub fn ziwei_chart(y: i64, m: i64, d: i64, hour: i64, minute: i64, gender: i64) -> ZiweiChart {
    // 晚子时换日（23 点起按下日算）
    let (ey, em, ed) = if hour >= 23 {
        civil_from_days(days_from_civil(y, m, d) + 1)
    } else {
        (y, m, d)
    };
    let solar = solar::from_ymdhms(ey, em, ed, hour % 24, minute, 0);
    let lunar = solar.get_lunar();
    let year_gz = lunar.get_year_in_gan_zhi();
    let year_gan = year_gz.chars().next().unwrap().to_string();
    let year_zhi = year_gz.chars().nth(1).unwrap().to_string();
    let ygi = ZW_GAN.iter().position(|x| *x == year_gan).unwrap() as i64;
    let yzi = ZW_ZHI.iter().position(|x| *x == year_zhi).unwrap() as i64;
    let raw_month = lunar.get_month();
    let is_leap = raw_month < 0;
    let lm = raw_month.abs();
    let ld = lunar.get_day();
    // 时支（子=0）
    let hz: i64 = if hour >= 23 { 0 } else { ((hour + 1) / 2) % 12 };
    // 命宫用月份索引：闰月且日 > 15 按下月
    let lm0 = (lm - 1) + if is_leap && ld > 15 { 1 } else { 0 };
    let p_soul = pmod(lm0 - hz, 12);
    let p_body = pmod(lm0 + hz, 12);
    // 宫干：五虎遁寅干 + 宫位偏移
    let tiger: i64 = match ygi {
        0 | 5 => 2,  // 甲己 丙寅
        1 | 6 => 4,  // 乙庚 戊寅
        2 | 7 => 6,  // 丙辛 庚寅
        3 | 8 => 8,  // 丁壬 壬寅
        _ => 0,      // 戊癸 甲寅
    };
    let palace_stem = |p: i64| -> i64 { pmod(tiger + p, 10) };
    let palace_branch = |p: i64| -> i64 { pmod(p + 2, 12) };
    // 五行局（命宫干支取数法）
    let soul_stem = palace_stem(p_soul);
    let soul_branch = palace_branch(p_soul);
    let stem_num = soul_stem / 2 + 1;
    let branch_num = (soul_branch % 6) / 2 + 1;
    let mut s = stem_num + branch_num;
    while s > 5 {
        s -= 5;
    }
    let (ju_name, ju_num): (&str, i64) = match s {
        1 => ("木三局", 3),
        2 => ("金四局", 4),
        3 => ("水二局", 2),
        4 => ("火六局", 6),
        _ => ("土五局", 5),
    };
    // 紫微定位
    let q = ju_num;
    let mut k = 0i64;
    while (ld + k) % q != 0 {
        k += 1;
    }
    let quotient = ((ld + k) / q) % 12;
    let p_zw = pmod(quotient - 1 + if k % 2 == 0 { k } else { -k }, 12);
    let p_tf = pmod(12 - p_zw, 12);

    // 星耀落宫：(宫位索引 p, 星名, 类别）
    let mut stars_at: Vec<Vec<ZwStar>> = vec![Vec::new(); 12];
    let hua = mutagens(&year_gan);
    let hua_of = |name: &str| -> String {
        for (i, hn) in hua.iter().enumerate() {
            if *hn == name {
                return ["禄", "权", "科", "忌"][i].to_string();
            }
        }
        String::new()
    };
    let mut put = |p: i64, name: &str, kind: &str, stars_at: &mut Vec<Vec<ZwStar>>| {
        let p = pmod(p, 12);
        stars_at[p as usize].push(ZwStar {
            name: name.to_string(),
            kind: kind.to_string(),
            brightness: brightness(name, p).to_string(),
            hua: hua_of(name),
        });
    };
    // 紫微系（逆行）
    for (off, name) in [(0, "紫微"), (-1, "天机"), (-3, "太阳"), (-4, "武曲"), (-5, "天同"), (-8, "廉贞")] {
        put(p_zw + off, name, "主星", &mut stars_at);
    }
    // 天府系（顺行）
    for (off, name) in [(0, "天府"), (1, "太阴"), (2, "贪狼"), (3, "巨门"), (4, "天相"), (5, "天梁"), (6, "七杀"), (10, "破军")] {
        put(p_tf + off, name, "主星", &mut stars_at);
    }
    // 辅星（先算地支，再转宫位索引 p = 支 - 2）
    let b2p = |b: i64| pmod(b - 2, 12);
    put(b2p(pmod(10 - hz, 12)), "文昌", "辅星", &mut stars_at);
    put(b2p(pmod(4 + hz, 12)), "文曲", "辅星", &mut stars_at);
    put(2 + lm0, "左辅", "辅星", &mut stars_at);
    put(8 - lm0, "右弼", "辅星", &mut stars_at);
    let (kui, yue) = match ygi {
        0 | 4 | 6 => (1, 7),
        1 | 5 => (0, 8),
        7 => (6, 2),
        2 | 3 => (11, 9),
        _ => (3, 5),
    };
    put(b2p(kui), "天魁", "辅星", &mut stars_at);
    put(b2p(yue), "天钺", "辅星", &mut stars_at);
    let lu_branch = match ygi {
        0 => 2, 1 => 3, 2 | 4 => 5, 3 | 5 => 6, 6 => 8, 7 => 9, 8 => 11, _ => 0,
    };
    put(b2p(lu_branch), "禄存", "辅星", &mut stars_at);
    put(b2p(lu_branch + 1), "擎羊", "煞星", &mut stars_at);
    put(b2p(lu_branch - 1), "陀罗", "煞星", &mut stars_at);
    let ma_branch = match yzi {
        8 | 0 | 4 => 2,
        2 | 6 | 10 => 8,
        5 | 9 | 1 => 11,
        _ => 5,
    };
    put(b2p(ma_branch), "天马", "辅星", &mut stars_at);
    let (huo_start, ling_start) = match yzi {
        2 | 6 | 10 => (1, 3),
        8 | 0 | 4 => (2, 10),
        5 | 9 | 1 => (3, 10),
        _ => (9, 10),
    };
    put(b2p(huo_start + hz), "火星", "煞星", &mut stars_at);
    put(b2p(ling_start + hz), "铃星", "煞星", &mut stars_at);
    put(b2p(pmod(11 - hz, 12)), "地空", "煞星", &mut stars_at);
    put(b2p(pmod(11 + hz, 12)), "地劫", "煞星", &mut stars_at);
    let luan_branch = pmod(3 - yzi, 12);
    put(b2p(luan_branch), "红鸾", "辅星", &mut stars_at);
    put(b2p(luan_branch + 6), "天喜", "辅星", &mut stars_at);

    // 大限：阳男阴女顺行（年支阴阳 × 性别）
    let year_yang = yzi % 2 == 0;
    let forward = year_yang == (gender == 1);
    let mut daxian_of: Vec<String> = vec![String::new(); 12];
    for step in 0..12i64 {
        let p = if forward { pmod(p_soul + step, 12) } else { pmod(p_soul - step, 12) };
        let start = ju_num + step * 10;
        daxian_of[p as usize] = format!("{}-{}", start, start + 9);
    }

    let gz_of = |p: i64| -> String {
        format!("{}{}", ZW_GAN[palace_stem(p) as usize], ZW_ZHI[palace_branch(p) as usize])
    };
    let mut palaces = Vec::new();
    for p in 0..12i64 {
        let name_idx = pmod(p_soul - p, 12);
        palaces.push(ZwPalace {
            branch: ZW_ZHI[palace_branch(p) as usize].to_string(),
            ganzhi: gz_of(p),
            name: PALACE_NAMES[name_idx as usize].to_string(),
            is_soul: p == p_soul,
            is_body: p == p_body,
            daxian: daxian_of[p as usize].clone(),
            stars: stars_at[p as usize].clone(),
        });
    }
    ZiweiChart {
        lunar_text: format!(
            "{}年{}月{}",
            lunar.get_year_in_chinese(),
            lunar.get_month_in_chinese(),
            lunar.get_day_in_chinese()
        ),
        year_ganzhi: year_gz,
        soul_ganzhi: gz_of(p_soul),
        body_ganzhi: gz_of(p_body),
        ju: ju_name.to_string(),
        ziwei_branch: ZW_ZHI[palace_branch(p_zw) as usize].to_string(),
        palaces,
    }
}
