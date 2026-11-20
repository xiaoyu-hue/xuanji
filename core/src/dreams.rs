//! 解梦：词条库为本站原创（content/dreams.json），核心只负责透传。

// ---------- 8. 解梦（本站原创词条，content/dreams.json） ----------

pub const DREAMS_JSON: &str = include_str!("../../content/dreams.json");

pub fn dreams_json() -> String {
    DREAMS_JSON.to_string()
}
