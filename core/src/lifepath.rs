//! 生命灵数（西方馆）：由出生日期算生命路径数等核心数字。
//! 数字约简保留大师数 11/22（灵数学通行口径）；解读文案为本站原创。

use serde::Serialize;

fn digit_sum(mut n: u32) -> u32 {
    let mut s = 0;
    while n > 0 {
        s += n % 10;
        n /= 10;
    }
    s
}

/// 约简到个位；keep_master 时 11/22 停步（大师数）
fn reduce(mut n: u32, keep_master: bool) -> u32 {
    while n > 9 {
        if keep_master && (n == 11 || n == 22) {
            break;
        }
        n = digit_sum(n);
    }
    n
}

/// (名称， 特质， 提醒）——本站原创
fn text_of(n: u32) -> (&'static str, &'static str, &'static str) {
    match n {
        1 => ("开创者", "独立、果断、天生带头，认准的事不等人推着走。", "别把「靠自己」过成「不求人」，开口求助不丢人。"),
        2 => ("协调者", "敏感细腻、擅长体察人心，是团队里的黏合剂。", "太在意别人感受时，先问问自己想要什么。"),
        3 => ("表达者", "点子多、感染力强，快乐和灵感都藏不住。", "热闹过后记得收尾，虎头蛇尾最可惜。"),
        4 => ("建设者", "踏实可靠、条理分明，喜欢把日子过成有章法的样子。", "计划之外偶尔留点空白，意外不全是坏事。"),
        5 => ("自由者", "好奇心旺盛、讨厌被框住，人生关键词是体验。", "自由不是逃避承诺，选定一件事扎下去看看。"),
        6 => ("守护者", "责任感强、重家庭与承诺，愿意为在乎的人兜底。", "别把付出变成账本，爱不是等价交换。"),
        7 => ("思索者", "爱钻研、重内省，表面安静，脑子里自成一个世界。", "想通了也要说出来，别人不会读心。"),
        8 => ("掌局者", "目标感和执行力都强，对资源与局面有天然嗅觉。", "赢很重要，但别把每段关系都过成谈判。"),
        9 => ("理想者", "共情力强、格局大，容易为「更大的意义」心动。", "先照顾好具体的人和事，再谈远方。"),
        11 => ("直觉者（大师数）", "直觉敏锐、感受力放大版的 2 号，常能察觉没说出口的东西。", "情绪潮汐大时，先睡一觉再做决定。"),
        22 => ("筑梦者（大师数）", "放大版的 4 号，能把宏大蓝图拆成一步步落地的工程。", "蓝图再大也从今天的第一步算起，别只活在图纸里。"),
        _ => ("", "", ""),
    }
}

#[derive(Serialize)]
pub struct LifeNum {
    pub n: u32,
    pub name: String,
    pub trait_: String,
    pub tip: String,
}

#[derive(Serialize)]
pub struct Lifepath {
    pub life_path: LifeNum,
    pub birthday: LifeNum,
    pub attitude: LifeNum,
    pub personal_year: LifeNum,
    pub target_year: i64,
}

fn ln(n: u32) -> LifeNum {
    let (name, t, tip) = text_of(n);
    LifeNum {
        n,
        name: name.to_string(),
        trait_: t.to_string(),
        tip: tip.to_string(),
    }
}

/// 生命灵数：出生日期 + 目标年份（算个人年数）。
pub fn lifepath(y: i64, m: i64, d: i64, target_year: i64) -> Lifepath {
    let total = digit_sum(y as u32) + digit_sum(m as u32) + digit_sum(d as u32);
    let lp = reduce(total, true);
    let bd = reduce(d as u32, true);
    let at = reduce(digit_sum(m as u32) + digit_sum(d as u32), false);
    let py = reduce(
        digit_sum(m as u32) + digit_sum(d as u32) + digit_sum(target_year as u32),
        false,
    );
    Lifepath {
        life_path: ln(lp),
        birthday: ln(bd),
        attitude: ln(at),
        personal_year: ln(py),
        target_year,
    }
}
