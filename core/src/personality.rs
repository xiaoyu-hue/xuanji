//! 16 型人格测试（心智馆）：48 道原创题、每维 12 题，核心层计分。
//! 2.1.0 曾上线、3.0.0 下线，5.5.0 按新架构恢复（题库与文案沿用原版原创）。
//! 注：本测试为通俗四维偏好测试，与 MBTI® 官方测评无关，页面如实标注。

/// 16 型人格题库与人格文案（文件：content/personality.json，构建时嵌入）
pub const PERSONALITY_JSON: &str = include_str!("../../content/personality.json");

/// 返回完整人格题库 JSON（题目 + 16 型文案）
pub fn personality_json() -> String {
    PERSONALITY_JSON.to_string()
}

/// 人格计分：answers 为 48 个字符的作答串（'a'/'b'，按题序）。
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
        let key = if chosen[i] == 'a' {
            "a"
        } else if chosen[i] == 'b' {
            "b"
        } else {
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
