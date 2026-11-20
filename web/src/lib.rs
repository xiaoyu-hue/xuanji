//! 玄机网页接口层（xuanji-web）：把核心各体系的计算暴露给网页调用。
//!
//! 约定：每文件一个体系的绑定；绑定只做 i32→i64 参数转换与 JSON 序列化，
//! 不写任何业务逻辑（业务全在 xuanji-core）。新增体系时在此登记一行。

mod almanac;
mod answers;
mod bazi;
mod bone;
mod dreams;
mod hehun;
mod qian;
mod wedding;
mod ziwei;

pub use almanac::almanac_json;
pub use answers::book_answer;
pub use bazi::bazi_json;
pub use bone::bone_json;
pub use dreams::dreams_json;
pub use hehun::hehun_json;
pub use qian::daily_qian_json;
pub use wedding::wedding_json;
pub use ziwei::ziwei_json;
