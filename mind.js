// 玄机页面编排器（心智馆）：只负责加载计算引擎、逐个初始化功能模块。
// 故障隔离约定与东方馆相同：每模块独立 try/catch，互不连坐。
import { loadWasm } from './js/wasm.js?v=6.1.0';
import { ctx, el } from './js/ctx.js?v=6.1.0';
import { initPersonality } from './js/personality.js?v=6.1.0';
import { initBigfive } from './js/bigfive.js?v=6.1.0';
import { initHolland } from './js/holland.js?v=6.1.0';

const FEATURES = [
  ['personality', '16 型人格', initPersonality],
  ['bigfive', '大五人格', initBigfive],
  ['holland', '霍兰德职业兴趣', initHolland],
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
  el('loading').textContent = '量表没准备好，请刷新重试';
  console.error(e);
});
