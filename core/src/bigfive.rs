//! 大五人格（心智馆）：IPIP 50 题大五标记量表（条目为公有领域，
//! 中文为本站翻译），五点计分、反向题倒置，核心层计分。
//! 结果按低/中/高三档给本站原创解读；本测验为科普性质，非临床评估。

use serde::Serialize;

/// IPIP 50 题（文件：content/bigfive.json，构建时嵌入）
pub const BIGFIVE_JSON: &str = include_str!("../../content/bigfive.json");

#[derive(serde::Deserialize)]
struct Item {
    n: u32,
    f: String,
    rev: bool,
    en: String,
    zh: String,
}

/// 返回题库 JSON（页面渲染用）
pub fn bigfive_json() -> String {
    BIGFIVE_JSON.to_string()
}

#[derive(Serialize)]
pub struct TraitScore {
    /// E 外向 / A 宜人 / C 尽责 / S 情绪稳定 / O 开放
    pub key: String,
    pub name: String,
    pub score: u32,
    pub band: String,
    pub text: String,
}

fn trait_meta(key: &str, band: &str) -> (&'static str, &'static str) {
    // (维度名， 分档解读）——本站原创
    match (key, band) {
        ("E", "高") => ("外向性", "能量从人群里来：热闹让你充电，主动、健谈、存在感强。适合站在台前和人打交道的场合。"),
        ("E", "中") => ("外向性", "动静皆宜：能热闹也能独处，看场合切换状态，是比较平衡的一档。"),
        ("E", "低") => ("外向性", "能量从独处里来：安静、内敛、深交胜过泛交。社交不是负担，但需要留时间回血。"),
        ("A", "高") => ("宜人性", "体贴热心、容易信任人，冲突里先想着体谅对方。暖是真的，也要留防人之心。"),
        ("A", "中") => ("宜人性", "友善有分寸：愿意帮人，也守得住自己的边界，不一味讨好。"),
        ("A", "低") => ("宜人性", "直率务实、不太迁就：说话办事对事不对人，有时显得冷，但靠谱不虚伪。"),
        ("C", "高") => ("尽责性", "自律条理、说到做到：计划感强，细节不放过，是别人眼里的定海神针。"),
        ("C", "中") => ("尽责性", "松紧适中：正事不掉链子，日常也能容忍一点乱和拖延。"),
        ("C", "低") => ("尽责性", "随性灵活、不爱被流程绑住：即兴发挥是强项，收尾和守时是需要刻意补的课。"),
        ("S", "高") => ("情绪稳定性", "情绪稳、抗压强：天塌下来先想办法，心态是你最大的本钱。"),
        ("S", "中") => ("情绪稳定性", "情绪总体可控：偶尔波动，但能自己消化，不太影响正事。"),
        ("S", "低") => ("情绪稳定性", "感受敏锐、容易多想：情绪起伏大不是缺点，是雷达太灵——学会给情绪留缓冲期。"),
        ("O", "高") => ("开放性", "好奇心与想象力旺盛：对新观念、艺术和抽象问题天然有兴趣，不喜欢一成不变。"),
        ("O", "中") => ("开放性", "新旧平衡：愿意尝鲜，也重视实际，不盲目追新也不守旧。"),
        ("O", "低") => ("开放性", "务实落地、偏爱具体：对抽象概念耐心有限，相信经验和看得见摸得着的东西。"),
        _ => ("", ""),
    }
}

/// 计分：answers 为 50 个字符的作答串（'1'..'5'，按题序）。
pub fn bigfive_score(answers: &str) -> String {
    let items: Vec<Item> = match serde_json::from_str(BIGFIVE_JSON) {
        Ok(v) => v,
        Err(e) => return format!("{{\"error\":\"题库解析失败：{e}\"}}"),
    };
    let chosen: Vec<char> = answers.chars().collect();
    if chosen.len() != items.len() {
        return format!("{{\"error\":\"作答长度应为 {}，实际 {}\"}}", items.len(), chosen.len());
    }
    let mut sums = std::collections::HashMap::new();
    for (it, ch) in items.iter().zip(chosen.iter()) {
        let v = match ch.to_digit(10) {
            Some(v @ 1..=5) => v,
            _ => return "{\"error\":\"作答只能包含 1-5\"}".to_string(),
        };
        let scored = if it.rev { 6 - v } else { v };
        *sums.entry(it.f.clone()).or_insert(0u32) += scored;
    }
    let mut traits = Vec::new();
    for key in ["E", "A", "C", "S", "O"] {
        let score = *sums.get(key).unwrap_or(&0);
        let band = if score >= 37 {
            "高"
        } else if score >= 24 {
            "中"
        } else {
            "低"
        };
        let (name, text) = trait_meta(key, band);
        traits.push(TraitScore {
            key: key.to_string(),
            name: name.to_string(),
            score,
            band: band.to_string(),
            text: text.to_string(),
        });
    }
    serde_json::json!({"traits": traits}).to_string()
}
