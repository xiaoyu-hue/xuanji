//! 卢恩符文（西方馆）：古弗萨克 24 卢恩，每日一卢恩固定、亦可随手抽三卢恩。
//! 牌义为本站原创；8 枚形制对称的卢恩（格布、哈格拉兹、瑙提兹、伊萨、
//! 耶拉、埃瓦兹、佩索、索维洛）按通行说法不设逆位。

use serde::Serialize;
use crate::common::{date_seed, hash64};

#[derive(Clone)]
pub struct Rune {
    pub ch: &'static str,
    pub name: &'static str,
    pub kw: &'static str,
    pub up: &'static str,
    pub rev: Option<&'static str>,
}

const R: Option<&str> = None;
const fn r(s: &'static str) -> Option<&'static str> {
    Some(s)
}

pub const RUNES: [Rune; 24] = [
    Rune { ch: "ᚠ", name: "费胡 Fehu", kw: "财富 · 流动", up: "资源正在流动，手里的东西要用起来、分享出去，守着不动反而贬值。", rev: r("钱财上有漏口，先堵漏再谈进账；借出的东西短期内难回。") },
    Rune { ch: "ᚢ", name: "乌鲁兹 Uruz", kw: "力量 · 野性", up: "体力和意志都在高点，适合硬碰硬地推进一件难事。", rev: r("状态不在巅峰，别硬扛，恢复比逞强重要。") },
    Rune { ch: "ᚦ", name: "苏里萨兹 Thurisaz", kw: "防御 · 棘刺", up: "先守后攻。有人或事在试探你的边界，亮明态度就是最好的回应。", rev: r("小心冲动反击，刺会先扎到自己；等对方先出招。") },
    Rune { ch: "ᚨ", name: "安苏兹 Ansuz", kw: "讯息 · 智慧", up: "留意一句话、一个消息，答案可能藏在别人的随口一提里。", rev: r("信息有噪音，别急着信；重要的话核实两遍。") },
    Rune { ch: "ᚱ", name: "莱多 Raidho", kw: "旅程 · 节奏", up: "事情在轨道上，保持节奏即可，不必加速也不必刹车。", rev: r("行程或计划有延误，留好备选方案，晚一点不等于到不了。") },
    Rune { ch: "ᚲ", name: "肯纳兹 Kenaz", kw: "火炬 · 洞见", up: "迷雾里亮起一盏灯：你会突然看清一件困扰已久的事的关键。", rev: r("暂时看不清方向不是坏事，别在暗处做重大决定。") },
    Rune { ch: "ᚹ", name: "温约 Wunjo", kw: "喜悦 · 和谐", up: "和气与喜悦自来，适合团聚、庆祝、把好消息分享出去。", rev: r("表面热闹心里别扭，喜悦要真的，不要演的。") },
    Rune { ch: "ᚷ", name: "格布 Gebo", kw: "礼物 · 交换", up: "有来有往的一天。接受善意，也把善意递出去，关系在交换里变近。", rev: R },
    Rune { ch: "ᚺ", name: "哈格拉兹 Hagalaz", kw: "冰雹 · 打断", up: "计划外的打断来了，它砸碎的往往是本来就该结束的东西。", rev: R },
    Rune { ch: "ᚾ", name: "瑙提兹 Nauthiz", kw: "需要 · 约束", up: "条件有限就打有限的牌，约束里练出的本事最扎实。", rev: R },
    Rune { ch: "ᛁ", name: "伊萨 Isa", kw: "冰 · 停滞", up: "冻住不是终点，是保存。眼下适合等，不适合推。", rev: R },
    Rune { ch: "ᛃ", name: "耶拉 Jera", kw: "收获 · 年轮", up: "之前种下的到了收的时候，按季节办事，不催熟。", rev: R },
    Rune { ch: "ᛇ", name: "埃瓦兹 Eihwaz", kw: "紫杉 · 坚韧", up: "像紫杉一样弯而不折，压力越大越显出韧性。", rev: R },
    Rune { ch: "ᛈ", name: "佩索 Perthro", kw: "命运 · 未知", up: "骰子已经掷下，未知里藏着比预期更好的可能，留点好奇心。", rev: R },
    Rune { ch: "ᛉ", name: "阿尔吉兹 Algiz", kw: "守护 · 警觉", up: "直觉在报警也在一旁守护，相信第一反应，远离让你不舒服的局。", rev: r("防线有缺口，重要东西（钱、账号、承诺）多检查一遍。") },
    Rune { ch: "ᛊ", name: "索维洛 Sowilo", kw: "太阳 · 胜利", up: "阳光正盛，适合把成果亮出来，胜负手就在今天。", rev: R },
    Rune { ch: "ᛏ", name: "提瓦兹 Tiwaz", kw: "正义 · 荣誉", up: "该争的理要争，站直了说话；公平会站在有准备的人这边。", rev: r("胜负心太重会伤和气，先想想赢了之后还要不要相处。") },
    Rune { ch: "ᛒ", name: "贝尔卡纳 Berkana", kw: "桦树 · 新生", up: "新的开始在发芽：新关系、新项目、新习惯，今天适合播种。", rev: r("新芽还嫩，别急着向全世界宣布，先养根。") },
    Rune { ch: "ᛖ", name: "埃瓦兹 Ehwaz", kw: "马 · 信任", up: "合作讲究同频，找到步调一致的人，路会走得又快又稳。", rev: r("搭档之间步调乱了，先对齐目标再谈速度。") },
    Rune { ch: "ᛗ", name: "曼纳兹 Mannaz", kw: "人 · 自我", up: "答案在人身上：求助、结盟、照镜子看清自己，都是今天的功课。", rev: r("别把希望全押在别人身上，这局先靠自己。") },
    Rune { ch: "ᛚ", name: "拉古兹 Laguz", kw: "水 · 直觉", up: "顺流而下，跟着感觉走一段；理性分析不出的时候，水知道方向。", rev: r("情绪起了漩涡，别在漩涡里做承诺，等水平稳下来。") },
    Rune { ch: "ᛜ", name: "英格瓦兹 Ingwaz", kw: "种子 · 潜能", up: "能量在内部积蓄，外表安静，里面正在发生重要的事。", rev: r("还没到破土的时候，继续蓄力，保密也是保护。") },
    Rune { ch: "ᛞ", name: "达格兹 Dagaz", kw: "黎明 · 觉醒", up: "天亮了：一个阶段彻底翻篇，适合用全新眼光看旧问题。", rev: r("黎明前最黑，转变已经在路上，别在最后一段放弃。") },
    Rune { ch: "ᛟ", name: "奥萨拉 Othala", kw: "家产 · 传承", up: "根基与归属是今天的主题，家里的事、长久的资产，值得认真对待。", rev: r("旧模式在拖后腿，传承不等于照搬，该断的习惯要断。") },
];

#[derive(Serialize)]
pub struct RuneDraw {
    pub ch: String,
    pub name: String,
    pub kw: String,
    pub reversed: bool,
    pub meaning: String,
    /// 三卢恩位置名（单抽为空）
    pub pos: String,
}

fn pick(h: u64, used: &[usize]) -> (usize, bool) {
    let mut x = hash64(h);
    let mut idx = (x % 24) as usize;
    while used.contains(&idx) {
        x = hash64(x ^ 0x51AB);
        idx = (x % 24) as usize;
    }
    let rune = &RUNES[idx];
    let reversed = rune.rev.is_some() && (x >> 8) % 2 == 1;
    (idx, reversed)
}

fn to_draw(idx: usize, reversed: bool, pos: &str) -> RuneDraw {
    let rune = &RUNES[idx];
    RuneDraw {
        ch: rune.ch.to_string(),
        name: rune.name.to_string(),
        kw: rune.kw.to_string(),
        reversed,
        meaning: if reversed {
            rune.rev.unwrap().to_string()
        } else {
            rune.up.to_string()
        },
        pos: pos.to_string(),
    }
}

/// 今日一卢恩：同一天抽到的符文（含正/逆位）固定不变
pub fn daily_rune(y: i64, m: i64, d: i64) -> RuneDraw {
    let (idx, rev) = pick(date_seed(y, m, d) ^ 0x9A11E, &[]);
    to_draw(idx, rev, "")
}

/// 三卢恩：过去 / 现在 / 未来，熵由调用方传入
pub fn runes_draw3(entropy: u64) -> Vec<RuneDraw> {
    let mut used = vec![];
    let mut out = vec![];
    for (i, pos) in ["过去", "现在", "未来"].iter().enumerate() {
        let (idx, rev) = pick(entropy ^ (i as u64).wrapping_mul(0x9E3779B97F4A7C15), &used);
        used.push(idx);
        out.push(to_draw(idx, rev, pos));
    }
    out
}
