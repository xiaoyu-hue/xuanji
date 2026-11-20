// 页面模块：紫微斗数。只负责本功能的页面交互，计算经 w（Rust 核心绑定）完成；
// 与其他功能模块不互相引用，共享物只从 ctx.js 取（低耦合约定）。
import { el } from './ctx.js?v=5.0.0';

export function initZiwei(w) {
// 紫微斗数
let zwGender = 1;
el('zwGender').querySelectorAll('button').forEach(b => b.onclick = () => {
  zwGender = Number(b.dataset.g);
  el('zwGender').querySelectorAll('button').forEach(x => x.classList.toggle('on', x === b));
});
const ZWPOS = { '巳': [1, 1], '午': [1, 2], '未': [1, 3], '申': [1, 4], '辰': [2, 1], '酉': [2, 4], '卯': [3, 1], '戌': [3, 4], '寅': [4, 1], '丑': [4, 2], '子': [4, 3], '亥': [4, 4] };
el('btnZiwei').onclick = () => {
  const dv = el('zwDate').value, tv = el('zwTime').value || '12:00';
  if (!dv) { alert('先选出生日期'); return; }
  const [y, m, d] = dv.split('-').map(Number);
  const [hh, mm] = tv.split(':').map(Number);
  const c = JSON.parse(w.ziwei_json(y, m, d, hh, mm, zwGender));
  const huaCls = { '禄': 'lu', '权': 'quan', '科': 'ke', '忌': 'ji' };
  let html = `<div class="zwcenter"><span class="big serif">玄 机 命 盘</span><span>农历${c.lunar_text}</span><span>${c.year_ganzhi}年 · 命宫${c.soul_ganzhi} · 身宫${c.body_ganzhi}</span><span>${c.ju} · 紫微在${c.ziwei_branch}宫</span></div>`;
  for (const p of c.palaces) {
    const [r, col] = ZWPOS[p.branch];
    const stars = p.stars.map(st => st.kind === '主星'
      ? `<span class="maj serif">${st.name}${st.brightness ? `<sup>${st.brightness}</sup>` : ''}${st.hua ? `<span class="hua ${huaCls[st.hua]}">${st.hua}</span>` : ''}</span>`
      : `<span class="${st.kind === '煞星' ? 'sha' : 'min'}">${st.name}${st.brightness ? `<sup>${st.brightness}</sup>` : ''}${st.hua ? `<span class="hua ${huaCls[st.hua]}">${st.hua}</span>` : ''}</span>`).join('');
    html += `<div class="zwcell ${p.is_soul ? 'soul' : ''}" style="grid-row:${r};grid-column:${col}"><div class="zh"><span class="gz">${p.ganzhi}</span><span>${p.branch}宫</span></div><div class="pname">${p.name}${p.is_body ? '<span class="bodytag">身宫</span>' : ''}</div><div class="stars">${stars || '<span class="min">（空宫）</span>'}</div><div class="dx">大限 ${p.daxian} 岁</div></div>`;
  }
  el('zwGrid').innerHTML = html;
  el('zwBox').classList.remove('hidden');
};
}
