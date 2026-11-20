// 页面模块：卢恩符文（西方馆）。今日一卢恩 + 三卢恩；计算经 w 完成。
import { el, ctx } from './ctx.js?v=6.0.1';

export function initRunes(w) {
  const one = (c, pos) => `
    <div class="runecell">
      ${pos ? `<div class="runepos">${pos}</div>` : ''}
      <div class="runeglyph serif ${c.reversed ? 'rrev' : ''}">${c.ch}</div>
      <div class="runename">${c.name}${c.reversed ? ' · 逆位' : ''}</div>
      <div class="runekw">${c.kw}</div>
      <p class="persdesc">${c.meaning}</p>
    </div>`;
  el('runeDaily').innerHTML = one(JSON.parse(w.daily_rune_json(ctx.Y, ctx.M, ctx.D)), '');
  el('btnRunes').onclick = () => {
    const arr = JSON.parse(w.runes_draw3_json(Math.random() * 2 ** 53));
    el('runeOut').innerHTML = `<div class="runes3">${arr.map(c => one(c, c.pos)).join('')}</div>`;
    el('runeOut').classList.remove('hidden');
  };
}
