// 页面模块：周易抽卦。只负责本功能的页面交互，计算经 w（Rust 核心绑定）完成；
// 与其他功能模块不互相引用，共享物只从 ctx.js 取（低耦合约定）。
import { el } from './ctx.js?v=5.4.0';

export function initZhouyi(w) {
// 周易抽卦（金钱卦）
el('btnZhouyi').onclick = () => {
  const seed = (Date.now() ^ Math.floor(Math.random() * 0x7fffffff)) | 0;
  const z = JSON.parse(w.zhouyi_draw_json(seed));
  // 卦象自上而下画（上爻在最上），变爻在右侧标 ○（阳变）/ ×（阴变）
  let fig = '';
  for (let i = 5; i >= 0; i--) {
    const l = z.lines[i];
    const bar = l.yang ? '<span class="yang"></span>' : '<span class="yin"><i></i><i></i></span>';
    const mark = l.changing ? `<span class="gmark">${l.yang ? '○' : '×'}</span>` : '<span class="gmark"></span>';
    fig += `<div class="gline">${bar}${mark}<span class="gname">${l.name}</span></div>`;
  }
  el('zyFigure').innerHTML = fig;
  const changes = z.lines.filter(l => l.changing);
  let html = `<div class="guatitle serif">第${z.ben.n}卦 · ${z.ben.full}</div>`
    + `<p class="guaci">卦辞：${z.ben.guaci}</p>`
    + `<p class="guaci">大象：${z.ben.xiang}</p>`
    + `<p class="dayi">${z.ben.dayi}</p>`;
  if (changes.length) {
    html += `<div class="bianbox"><div class="biantitle">变爻</div>`
      + changes.map(l => `<p class="guaci">${l.name}：${l.text}</p>`).join('') + `</div>`;
  }
  if (z.yong) html += `<p class="guaci">用：${z.yong}</p>`;
  if (z.bian) {
    html += `<p class="guaci bianline">之卦：第${z.bian.n}卦 · ${z.bian.full} —— ${z.bian.guaci}</p>`
      + `<p class="dayi">${z.bian.dayi}</p>`;
  } else {
    html += `<p class="guaci bianline">六爻安静，无变卦，专看本卦。</p>`;
  }
  el('zyText').innerHTML = html;
  el('zyBox').classList.remove('hidden');
  el('zyBox').classList.remove('fade'); void el('zyBox').offsetWidth; el('zyBox').classList.add('fade');
};
}
