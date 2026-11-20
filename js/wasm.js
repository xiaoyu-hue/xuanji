// 计算引擎加载器：全站只加载一次 WASM（Rust 核心编译产物），
// 各功能模块共用同一份绑定对象 w。
import init, * as w from '../pkg/xuanji_web.js?v=6.0.1';

export async function loadWasm() {
  await init(new URL('../pkg/xuanji_web_bg.wasm?v=6.0.1', import.meta.url));
  return w;
}
