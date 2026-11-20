// 页面模块：称骨。只负责本功能的页面交互，计算经 w（Rust 核心绑定）完成；
// 与其他功能模块不互相引用，共享物只从 ctx.js 取（低耦合约定）。
import { el } from './ctx.js?v=5.0.1';

export function initBone(w) {
// 称骨
el('btnBone').onclick = () => {
  const dv = el('cgDate').value, tv = el('cgTime').value || '12:00';
  if (!dv) { alert('先选出生日期'); return; }
  const [y, m, d] = dv.split('-').map(Number);
  const [hh, mm] = tv.split(':').map(Number);
  const b = JSON.parse(w.bone_json(y, m, d, hh, mm));
  const fmtW = w => w >= 10 ? `${Math.floor(w / 10)}两${w % 10 ? w % 10 + '钱' : ''}` : `${w}钱`;
  el('boneMeta').innerHTML = `农历${b.lunar_text} · ${b.year_ganzhi}年 · ${b.hour_zhi}`;
  el('boneParts').innerHTML = [['年骨', b.year_w], ['月骨', b.month_w], ['日骨', b.day_w], ['时骨', b.hour_w]].map(([n, w]) =>
    `<div class="row"><label>${n}</label><div class="bar"><i style="width:${Math.round(w / 19 * 100)}%"></i></div><b>${fmtW(w)}</b></div>`).join('');
  el('boneTotal').textContent = `总骨重 ${b.total_text}`;
  el('boneVerse').textContent = b.verse;
  el('boneBox').classList.remove('hidden');
};
}
