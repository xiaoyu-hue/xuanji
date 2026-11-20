//! 犯太岁：按农历年生肖与本人生肖的地支关系（值 / 冲 / 刑 / 害 / 破）查询。
//!
//! 关系表为传统地支关系（已与多个公开来源交叉核对）：
//! 六冲对宫、子卯相刑、寅巳申与丑戌未两组三刑、辰午酉亥自刑、
//! 六害（子未、丑午、寅巳、卯辰、申亥、酉戌）、
//! 六破（子酉、卯午、辰丑、未戌、寅亥、巳申）。
//! 说明文字全部原创，只作民俗文化介绍，不作吉凶断言。

use serde::Serialize;

const ZODIAC: [&str; 12] = ["鼠", "牛", "虎", "兔", "龙", "蛇", "马", "羊", "猴", "鸡", "狗", "猪"];
const GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
const ZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

/// 六害（无序对，下标为生肖序：鼠0 … 猪11）
const HAI: [(usize, usize); 6] = [(0, 7), (1, 6), (2, 5), (3, 4), (8, 11), (9, 10)];
/// 六破（无序对）
const PO: [(usize, usize); 6] = [(0, 9), (3, 6), (1, 4), (7, 10), (2, 11), (5, 8)];
/// 两组三刑（寅巳申、丑戌未）
const XING_GROUPS: [[usize; 3]; 2] = [[2, 5, 8], [1, 10, 7]];
/// 自刑生肖下标（辰、午、酉、亥）
const ZI_XING: [usize; 4] = [4, 6, 9, 11];

fn in_pair(pairs: &[(usize, usize)], a: usize, b: usize) -> bool {
    pairs.iter().any(|&(x, y)| (x == a && y == b) || (x == b && y == a))
}

/// 本人下标 p 与太岁（年份生肖）下标 t 的全部关系，按传统分量从重到轻排序。
fn relations(t: usize, p: usize) -> Vec<&'static str> {
    let mut out: Vec<&'static str> = Vec::new();
    if p == t {
        out.push("值太岁");
        if ZI_XING.contains(&p) {
            out.push("刑太岁");
        }
        return out;
    }
    if (p + 6) % 12 == t {
        out.push("冲太岁");
    }
    let xing = in_pair(&[(0, 3)], p, t)
        || XING_GROUPS.iter().any(|g| g.contains(&p) && g.contains(&t));
    if xing {
        out.push("刑太岁");
    }
    if in_pair(&HAI, p, t) {
        out.push("害太岁");
    }
    if in_pair(&PO, p, t) {
        out.push("破太岁");
    }
    out
}

fn kind_text(kind: &str) -> &'static str {
    match kind {
        "值太岁" => "值太岁就是本命年。老话说这一年宜守不宜攻，大事缓一缓、步子稳一点。",
        "冲太岁" => "冲太岁与太岁正对面。老话说这一年变动多，搬迁、远行、换跑道都容易赶上，心里先有个谱。",
        "刑太岁" => "刑太岁主摩擦纠缠。老话说这一年说话办事多留余地，文书合同看仔细，少争一时之气。",
        "害太岁" => "害太岁又叫相穿，主暗地里的损耗。老话说这一年提防小人使绊，合作先讲清规矩再谈交情。",
        "破太岁" => "破太岁分量最轻，主小破小耗。老话说这一年看好钱袋子，小破财、小磕碰别太往心里去。",
        _ => "",
    }
}

#[derive(Serialize)]
pub struct TaisuiHit {
    pub zodiac: String,
    pub kinds: Vec<String>,
}

#[derive(Serialize)]
pub struct TaisuiYear {
    pub year: i64,
    pub year_ganzhi: String,
    pub year_zodiac: String,
    /// 这一年犯太岁的全部生肖（按生肖序）
    pub hits: Vec<TaisuiHit>,
    /// 查询者本人的关系（空 = 不犯）
    pub mine: Vec<String>,
    /// 查询者的说明文字（原创，老话体）
    pub mine_text: String,
}

/// 犯太岁查询：year 为公历年份（按农历年生肖论，公元 4 年为甲子鼠年起排）；
/// zodiac 为查询者生肖单字，空串或非法值表示只看全年名单。
pub fn taisui(year: i64, zodiac: &str) -> TaisuiYear {
    let t = (year - 4).rem_euclid(12) as usize;
    let ganzhi = format!(
        "{}{}",
        GAN[(year - 4).rem_euclid(10) as usize],
        ZHI[t]
    );
    let mut hits = Vec::new();
    for p in 0..12 {
        let kinds = relations(t, p);
        if !kinds.is_empty() {
            hits.push(TaisuiHit {
                zodiac: ZODIAC[p].to_string(),
                kinds: kinds.iter().map(|s| s.to_string()).collect(),
            });
        }
    }
    let (mine, mine_text) = match ZODIAC.iter().position(|&z| z == zodiac) {
        Some(p) => {
            let kinds = relations(t, p);
            let text = if kinds.is_empty() {
                "这一年与太岁不冲不刑、不害不破，老话说这叫安稳年，照常过日子就是福。".to_string()
            } else {
                kind_text(kinds[0]).to_string()
            };
            (kinds.iter().map(|s| s.to_string()).collect::<Vec<_>>(), text)
        }
        None => (Vec::new(), String::new()),
    };
    TaisuiYear {
        year,
        year_ganzhi: ganzhi,
        year_zodiac: ZODIAC[t].to_string(),
        hits,
        mine,
        mine_text,
    }
}
