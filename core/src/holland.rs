//! 霍兰德职业兴趣（心智馆）：48 道原创题项（RIASEC 六型各 8 题），
//! 五点计分，输出六型分数与前三兴趣代码。题项与解读均为本站原创。

/// 霍兰德题库（文件：content/holland.json，构建时嵌入）
pub const HOLLAND_JSON: &str = include_str!("../../content/holland.json");

/// 返回题库 JSON（页面渲染用）
pub fn holland_json() -> String {
    HOLLAND_JSON.to_string()
}

/// 计分：answers 为 48 个字符的作答串（'1'..'5'，按题序）。
/// 返回 {"types":[{key,name,score}...按分降序], "code":"前三字母"}。
pub fn holland_score(answers: &str) -> String {
    let bank: serde_json::Value = match serde_json::from_str(HOLLAND_JSON) {
        Ok(v) => v,
        Err(e) => return format!("{{\"error\":\"题库解析失败：{e}\"}}"),
    };
    let items = match bank["items"].as_array() {
        Some(a) => a.clone(),
        None => return "{\"error\":\"题库格式错误\"}".to_string(),
    };
    let chosen: Vec<char> = answers.chars().collect();
    if chosen.len() != items.len() {
        return format!("{{\"error\":\"作答长度应为 {}，实际 {}\"}}", items.len(), chosen.len());
    }
    let mut sums: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    for (it, ch) in items.iter().zip(chosen.iter()) {
        let v = match ch.to_digit(10) {
            Some(v @ 1..=5) => v,
            _ => return "{\"error\":\"作答只能包含 1-5\"}".to_string(),
        };
        let f = it["f"].as_str().unwrap_or("").to_string();
        *sums.entry(f).or_insert(0) += v;
    }
    // 固定顺序 RIASEC 作同分裁决（保持稳定）
    let order = ["R", "I", "A", "S", "E", "C"];
    let mut rows: Vec<(usize, &str, u32)> = order
        .iter()
        .enumerate()
        .map(|(i, k)| (i, *k, *sums.get(*k).unwrap_or(&0)))
        .collect();
    rows.sort_by(|a, b| b.2.cmp(&a.2).then(a.0.cmp(&b.0)));
    let types: Vec<serde_json::Value> = rows
        .iter()
        .map(|(_, k, s)| {
            let t = &bank["types"][k];
            serde_json::json!({
                "key": k, "name": t["name"], "score": s,
                "text": t["text"], "jobs": t["jobs"]
            })
        })
        .collect();
    let code: String = rows.iter().take(3).map(|r| r.1).collect();
    serde_json::json!({"types": types, "code": code}).to_string()
}
