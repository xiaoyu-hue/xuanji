// 玄机页面编排器（东方馆）：只负责加载计算引擎、逐个初始化功能模块。
// 故障隔离约定：每个模块独立 try/catch，一个模块出错只在它自己的卡片里
// 提示，绝不连坐全页（4.0.1 全页失效的教训，见 docs/架构文档.md）。
import { loadWasm } from './js/wasm.js?v=6.1.0';
import { ctx, el } from './js/ctx.js?v=6.1.0';
import { initAlmanac } from './js/almanac.js?v=6.1.0';
import { initQian } from './js/qian.js?v=6.1.0';
import { initZhouyi } from './js/zhouyi.js?v=6.1.0';
import { initMeihua } from './js/meihua.js?v=6.1.0';
import { initLiuyao } from './js/liuyao.js?v=6.1.0';
import { initFeixing } from './js/feixing.js?v=6.1.0';
import { initQimen } from './js/qimen.js?v=6.1.0';
import { initWedding } from './js/wedding.js?v=6.1.0';
import { initBazi } from './js/bazi.js?v=6.1.0';
import { initBone } from './js/bone.js?v=6.1.0';
import { initHehun } from './js/hehun.js?v=6.1.0';
import { initTaisui } from './js/taisui.js?v=6.1.0';
import { initDreams } from './js/dreams.js?v=6.1.0';
import { initZiwei } from './js/ziwei.js?v=6.1.0';
import { initShare } from './js/share.js?v=6.1.0';

const FEATURES = [
  ['almanac', '今日黄历', initAlmanac],
  ['qian', '今日一签', initQian],
  ['zhouyi', '周易抽卦', initZhouyi],
  ['meihua', '梅花易数', initMeihua],
  ['liuyao', '六爻', initLiuyao],
  ['wedding', '万事择日', initWedding],
  ['bazi', '八字排盘', initBazi],
  ['bone', '称骨', initBone],
  ['hehun', '合婚', initHehun],
  ['taisui', '犯太岁', initTaisui],
  ['dreams', '解梦', initDreams],
  ['feixing', '九宫飞星', initFeixing],
  ['ziwei', '紫微斗数', initZiwei],
  ['qimen', '奇门遁甲', initQimen],
  ['share', '分享图', initShare],
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
  el('loading').textContent = '黄历没翻开，请刷新重试';
  console.error(e);
});
