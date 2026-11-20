// 玄机页面编排器（西方馆）：只负责加载计算引擎、逐个初始化功能模块。
// 故障隔离约定与东方馆相同：每模块独立 try/catch，互不连坐。
import { loadWasm } from './js/wasm.js?v=6.0.0';
import { ctx, el } from './js/ctx.js?v=6.0.0';
import { initHoroscope } from './js/horoscope.js?v=6.0.0';
import { initTarot } from './js/tarot.js?v=6.0.0';
import { initLifepath } from './js/lifepath.js?v=6.0.0';
import { initRunes } from './js/runes.js?v=6.0.0';
import { initAnswers } from './js/answers.js?v=6.0.0';

const FEATURES = [
  ['horoscope', '星座运势', initHoroscope],
  ['tarot', '今日塔罗', initTarot],
  ['lifepath', '生命灵数', initLifepath],
  ['runes', '卢恩符文', initRunes],
  ['answers', '答案之书', initAnswers],
];

async function main() {
  const w = await loadWasm();
  el('todayLine').textContent = `${ctx.Y} 年 ${ctx.M} 月 ${ctx.D} 日 · 星期${ctx.week}`;
  el('loading').classList.add('hidden');
  el('app').classList.remove('hidden');
  for (const [key, name, init] of FEATURES) {
    try {
      init(w);
    } catch (e) {
      console.error(`[${name}] 初始化失败`, e);
      const card = document.querySelector(`[data-mod="${key}"]`);
      if (card) {
        const p = document.createElement('p');
        p.className = 'fineprint';
        p.textContent = `（${name}暂时打不开，刷新试试；其他功能不受影响）`;
        card.appendChild(p);
      }
    }
  }
}

main().catch(e => {
  el('loading').textContent = '星盘没铺开，请刷新重试';
  console.error(e);
});
