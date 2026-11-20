// 九宫飞星：流年飞星盘（立春为界，2 月 4 日前按上一年）
import { el } from './ctx.js?v=6.1.0';

function currentFxYear() {
  const n = new Date();
  return n.getMonth() === 0 || (n.getMonth() === 1 && n.getDate() < 4) ? n.getFullYear() - 1 : n.getFullYear();
}

export function initFeixing(w) {
  const input = el('fxYear');
  input.value = currentFxYear();
  const run = () => {
    const y = parseInt(input.value, 10);
    if (!y || y < 1900 || y > 2100) return;
    const f = JSON.parse(w.feixing_json(y));
    el('fxTitle').textContent = `${f.year} · ${f.ganzhi}年 · ${f.palaces[4].star_name}入中宫`;
    el('fxGrid').innerHTML = f.palaces.map(p => `
      <div class="fxcell ${p.luck === '吉' ? 'fxgood' : 'fxbad'}${p.dir === '中' ? ' fxcenter' : ''}">
        <div class="fxdir">${p.dir} · ${p.palace}</div>
        <div class="fxstar">${p.star}</div>
        <div class="fxname">${p.star_name}</div>
      </div>`).join('');
    el('fxNotes').innerHTML = f.palaces
      .map(p => `<div class="fxnote"><b>${p.dir} ${p.star_name}</b>（${p.luck}）${p.note}</div>`)
      .join('');
  };
  el('btnFeixing').addEventListener('click', run);
  run();
}
