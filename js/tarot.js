// 页面模块：今日塔罗（西方馆）。22 张大阿卡纳，含正/逆位；计算经 w 完成。
import { el, ctx } from './ctx.js?v=6.1.0';

const ROMAN = ['0', 'I', 'II', 'III', 'IV', 'V', 'VI', 'VII', 'VIII', 'IX', 'X', 'XI', 'XII', 'XIII', 'XIV', 'XV', 'XVI', 'XVII', 'XVIII', 'XIX', 'XX', 'XXI'];

export function initTarot(w) {
  function render(c) {
    el('tarotOut').innerHTML = `
      <div class="tcard ${c.reversed ? 'trev' : ''}"><div class="tnum serif">${ROMAN[c.num]}</div>
      <div class="tname serif">${c.name}</div><div class="ten">${c.en}</div></div>
      <p class="persdesc"><b>${c.reversed ? '逆位' : '正位'}</b> · ${c.kw}</p>
      <p class="persdesc">${c.meaning}</p>`;
    el('tarotOut').classList.remove('hidden');
  }
  el('btnTarotDaily').onclick = () => render(JSON.parse(w.daily_tarot_json(ctx.Y, ctx.M, ctx.D)));
  el('btnTarotDraw').onclick = () => render(JSON.parse(w.tarot_draw_json(Math.random() * 2 ** 53)));
}
