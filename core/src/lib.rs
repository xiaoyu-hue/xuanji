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
pub mod feixing;
pub mod hehun;
pub mod hexagram;
pub mod liuyao;
pub mod qimen;
pub mod meihua;
pub mod zhouyi;
pub mod qian;
pub mod taisui;
pub mod wedding;
pub mod ziwei;
pub mod horoscope;
pub mod tarot;
pub mod lifepath;
pub mod runes;
pub mod personality;
pub mod bigfive;
pub mod holland;
pub mod natal;

pub use almanac::{almanac, day_hours, Almanac, HourLuck};
pub use answers::book_answer;
pub use bazi::{bazi, Bazi, DaYunItem, Pillar, WxCount};
pub use bone::{bone_weight, BoneWeight};
pub use dreams::{dreams_json, DREAMS_JSON};
pub use feixing::{feixing, Feixing, FxPalace};
pub use hehun::{hehun, MatchFactor, MatchResult};
pub use liuyao::{liuyao_cast, liuyao_from_values, LiuyaoResult, LyBianLine, LyLine};
pub use qimen::{qimen_chart, QimenChart, QmPalace};
pub use qian::{daily_qian, Qian};
pub use taisui::{taisui, TaisuiHit, TaisuiYear};
pub use meihua::{meihua, MeihuaResult};
pub use wedding::{day_search, wedding_days, WeddingDay};
pub use zhouyi::{build_from_values, zhouyi_draw, HexView, YaoLine, ZhouyiDraw};
pub use ziwei::{ziwei_chart, ziwei_liunian, LiuHua, LiuNian, LiuPalace, ZiweiChart, ZwPalace, ZwStar};
pub use horoscope::{zodiac_fortune, zodiac_index, Fortune, ZODIAC_NAMES};
pub use tarot::{daily_tarot, tarot_draw};
pub use lifepath::{lifepath, Lifepath};
pub use runes::{daily_rune, runes_draw3, RuneDraw};
pub use personality::{personality_json, personality_score};
pub use bigfive::{bigfive_json, bigfive_score};
pub use holland::{holland_json, holland_score};
pub use natal::{natal_chart, NatalChart};
