//! 玄机核心（xuanji-core）：民俗各体系的计算层，纯 Rust、无 IO。
//!
//! 分层与模块化约定（5.0.0 起）：
//! - `common` 是跨体系共用层，只放无业务含义的纯工具（日期换算、哈希）；
//! - 其余每个文件一个体系，模块之间互不调用对方内部实现，只经 `common` 共享；
//! - 内容数据在 `content/` 下按体系分文件，编译期嵌入各自模块；
//! - 对外 API 在此统一 re-export，调用方（web 绑定、自检程序）不感知内部分文件。

pub mod almanac;
pub mod answers;
pub mod bazi;
pub mod bone;
pub mod common;
pub mod dreams;
pub mod hehun;
pub mod qian;
pub mod taisui;
pub mod wedding;
pub mod ziwei;

pub use almanac::{almanac, day_hours, Almanac, HourLuck};
pub use answers::book_answer;
pub use bazi::{bazi, Bazi, DaYunItem, Pillar, WxCount};
pub use bone::{bone_weight, BoneWeight};
pub use dreams::{dreams_json, DREAMS_JSON};
pub use hehun::{hehun, MatchFactor, MatchResult};
pub use qian::{daily_qian, Qian};
pub use taisui::{taisui, TaisuiHit, TaisuiYear};
pub use wedding::{day_search, wedding_days, WeddingDay};
pub use ziwei::{ziwei_chart, ziwei_liunian, LiuHua, LiuNian, LiuPalace, ZiweiChart, ZwPalace, ZwStar};
