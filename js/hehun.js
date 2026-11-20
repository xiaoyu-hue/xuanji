// 页面模块：合婚。只负责本功能的页面交互，计算经 w（Rust 核心绑定）完成；
// 与其他功能模块不互相引用，共享物只从 ctx.js 取（低耦合约定）。
import { el } from './ctx.js?v=5.3.0';

export function initHehun(w) {
// 合婚
el('btnHehun').onclick = () => {
  const ga = (id) => el(id).value, tm = (id) => (el(id).value || '12:00').split(':').map(Number);
  if (!ga('hhADate') || !ga('hhBDate')) { alert('先把两个人的出生日期选好'); return; }
  const [ay, am, ad] = ga('hhADate').split('-').map(Number);
  const [by, bm, bd] = ga('hhBDate').split('-').map(Number);
  const [ah, amin] = tm('hhATime'), [bh, bmin] = tm('hhBTime');
  const r = JSON.parse(w.hehun_json(ay, am, ad, ah, amin, by, bm, bd, bh, bmin));
  el('hehunScore').textContent = `${r.score} 分 · ${r.grade}`;
  el('hehunFactors').innerHTML = r.factors.map(f =>
    `<div class="factor"><div class="fh"><b>${f.name}：${f.result}</b><span class="delta ${f.delta >= 0 ? 'up' : 'down'}">${f.delta >= 0 ? '+' : ''}${f.delta}</span></div><p>${f.note}</p></div>`).join('');
  el('hehunBox').classList.remove('hidden');
};
}
