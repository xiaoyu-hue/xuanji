//! 解梦绑定：只做参数类型转换（i32 → i64）与 JSON 序列化，不含业务逻辑。

use wasm_bindgen::prelude::*;

/// 解梦词条库（原创），返回 JSON
#[wasm_bindgen]
pub fn dreams_json() -> String {
    xuanji_core::dreams_json()
}
