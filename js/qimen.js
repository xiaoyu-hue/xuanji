// 奇门遁甲：时家拆补转盘排盘
import { el, ctx, pad } from './ctx.js?v=6.0.1';

export function initQimen(w) {
  const dateEl = el('qmDate');
  const hourEl = el('qmHour');
  dateEl.value = `${ctx.Y}-${pad(ctx.M)}-${pad(ctx.D)}`;
  hourEl.value = new Date().getHours();
  const run = () => {
    const [y, m, d] = dateEl.value.split('-').map(Number);
    const h = parseInt(hourEl.value, 10);
    if (!y || !m || !d || Number.isNaN(h)) return;
    const c = JSON.parse(w.qimen_json(y, m, d, h));
    el('qmTitle').textContent =
      `${c.jieqi} · ${c.dun}${c.ju}局 · ${c.yuan} · 符头${c.futou}`;
    el('qmSub').textContent =
      `日${c.day_ganzhi} 时${c.hour_ganzhi}（${c.xun}旬）· 值符${c.zhifu_star}落${c.zhifu_palace}宫 · 值使${c.zhishi_door}落${c.zhishi_palace}宫 · 旬空${c.kong_wang.join('')} · 驿马${c.horse_zhi}（${c.horse_palace}宫）`;
    el('qmGrid').innerHTML = c.palaces.map(p => `
      <div class="qmcell${p.num === 5 ? ' qmcenter' : ''}">
        <div class="qmdir">${p.dir} · ${p.name}${p.num}宫${p.horse ? ' · 驿马' : ''}${p.void.length ? ` · 空${p.void.join('')}` : ''}</div>
        ${p.num === 5
          ? `<div class="qmearth">地 ${p.earth}</div><div class="qmhint">中五寄坤</div>`
          : `<div class="qmgod">${p.god || ''}</div>
             <div class="qmstar">${p.star}</div>
             <div class="qmdoor">${p.door || ''}</div>
             <div class="qmtian">天 ${p.heaven.join('')}${p.heaven_hosted ? ` ${p.heaven_hosted}寄` : ''}</div>
             <div class="qmearth">地 ${p.earth}${p.hosted_earth ? ` ${p.hosted_earth}寄` : ''}</div>`}
      </div>`).join('');
  };
  el('btnQimen').addEventListener('click', run);
  run();
}
