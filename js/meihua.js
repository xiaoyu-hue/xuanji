// 页面模块：梅花易数。只负责本功能的页面交互，计算经 w（Rust 核心绑定）完成；
// 与其他功能模块不互相引用，共享物只从 ctx.js 取（低耦合约定）。
import { el } from './ctx.js?v=6.0.0';

export function initMeihua(w) {
// 梅花易数（时间起卦）
el('btnMeihua').onclick = () => {
  const now = new Date();
  const r = JSON.parse(w.meihua_json(now.getFullYear(), now.getMonth() + 1, now.getDate(), now.getHours()));
  let fig = '';
  for (let i = 5; i >= 0; i--) {
    const yang = r.ben_lines[i] === 1;
    const bar = yang ? '<span class="yang"></span>' : '<span class="yin"><i></i><i></i></span>';
    const mark = (i + 1 === r.dong_yao) ? '<span class="gmark">●</span>' : '<span class="gmark"></span>';
    fig += `<div class="gline">${bar}${mark}</div>`;
  }
  el('mhFigure').innerHTML = fig;
  el('mhText').innerHTML =
    `<p class="guaci" style="text-align:center">起卦时刻：农历${r.lunar_text}</p>`
    + `<div class="guatitle serif">第${r.ben.n}卦 · ${r.ben.full}</div>`
    + `<p class="guaci">卦辞：${r.ben.guaci}</p>`
    + `<p class="dayi">${r.ben.dayi}</p>`
    + `<div class="bianbox"><div class="biantitle">动爻 · 第${r.dong_yao}爻 ${r.dong_name}</div>`
    + `<p class="guaci">${r.dong_text}</p>`
    + `<p class="guaci bianline">变卦：第${r.bian.n}卦 · ${r.bian.full} —— ${r.bian.guaci}</p></div>`
    + `<p class="guaci">体用：体卦「${r.ti_gua}」、用卦「${r.yong_gua}」—— <b>${r.relation}</b></p>`
    + `<p class="dayi">${r.verdict}</p>`;
  el('mhBox').classList.remove('hidden');
  el('mhBox').classList.remove('fade'); void el('mhBox').offsetWidth; el('mhBox').classList.add('fade');
};
}
