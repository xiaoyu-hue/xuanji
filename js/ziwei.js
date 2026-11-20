// 页面模块：紫微斗数。只负责本功能的页面交互，计算经 w（Rust 核心绑定）完成；
// 与其他功能模块不互相引用，共享物只从 ctx.js 取（低耦合约定）。
import { el, ctx } from './ctx.js?v=6.1.0';

export function initZiwei(w) {
// 紫微斗数
let zwGender = 1;
el('zwGender').querySelectorAll('button').forEach(b => b.onclick = () => {
  zwGender = Number(b.dataset.g);
  el('zwGender').querySelectorAll('button').forEach(x => x.classList.toggle('on', x === b));
});
const ZWPOS = { '巳': [1, 1], '午': [1, 2], '未': [1, 3], '申': [1, 4], '辰': [2, 1], '酉': [2, 4], '卯': [3, 1], '戌': [3, 4], '寅': [4, 1], '丑': [4, 2], '子': [4, 3], '亥': [4, 4] };
const huaCls = { '禄': 'lu', '权': 'quan', '科': 'ke', '忌': 'ji' };
let lastChart = null;   // 最近一次命盘（流年叠加用）
let lastBirth = null;   // 最近一次排盘的出生参数

function birthArgs() {
  const dv = el('zwDate').value, tv = el('zwTime').value || '12:00';
  if (!dv) return null;
  const [y, m, d] = dv.split('-').map(Number);
  const [hh, mm] = tv.split(':').map(Number);
  return [y, m, d, hh, mm, zwGender];
}

function render(c, liu) {
  const liuName = {};   // 地支 → 流年宫名
  if (liu) for (const lp of liu.palaces) liuName[lp.branch] = lp.name;
  let html = `<div class="zwcenter"><span class="big serif">玄 机 命 盘</span><span>农历${c.lunar_text}</span><span>${c.year_ganzhi}年 · 命宫${c.soul_ganzhi} · 身宫${c.body_ganzhi}</span><span>${c.ju} · 紫微在${c.ziwei_branch}宫</span>${liu ? `<span class="liuyear">${liu.year} ${liu.year_ganzhi}年流年</span>` : ''}</div>`;
  for (const p of c.palaces) {
    const [r, col] = ZWPOS[p.branch];
    const stars = p.stars.map(st => st.kind === '主星'
      ? `<span class="maj serif">${st.name}${st.brightness ? `<sup>${st.brightness}</sup>` : ''}${st.hua ? `<span class="hua ${huaCls[st.hua]}">${st.hua}</span>` : ''}</span>`
      : `<span class="${st.kind === '煞星' ? 'sha' : 'min'}">${st.name}${st.brightness ? `<sup>${st.brightness}</sup>` : ''}${st.hua ? `<span class="hua ${huaCls[st.hua]}">${st.hua}</span>` : ''}</span>`).join('');
    const isLiuSoul = liu && p.branch === liu.soul_branch;
    html += `<div class="zwcell ${p.is_soul ? 'soul' : ''} ${isLiuSoul ? 'liusoul' : ''}" style="grid-row:${r};grid-column:${col}"><div class="zh"><span class="gz">${p.ganzhi}</span><span>${p.branch}宫</span></div><div class="pname">${p.name}${p.is_body ? '<span class="bodytag">身宫</span>' : ''}</div><div class="stars">${stars || '<span class="min">（空宫）</span>'}</div><div class="dx">大限 ${p.daxian} 岁</div>${liu ? `<div class="liuname">${liuName[p.branch] || ''}</div>` : ''}</div>`;
  }
  el('zwGrid').innerHTML = html;
  el('zwBox').classList.remove('hidden');
}

el('btnZiwei').onclick = () => {
  const args = birthArgs();
  if (!args) { alert('先选出生日期'); return; }
  lastBirth = args;
  lastChart = JSON.parse(w.ziwei_json(...args));
  el('zwYear').value = ctx.Y;
  el('zwLiuText').textContent = '';
  render(lastChart, null);
};

el('btnLiunian').onclick = () => {
  const args = lastBirth || birthArgs();
  if (!args) { alert('先选出生日期'); return; }
  if (!lastChart) { lastBirth = args; lastChart = JSON.parse(w.ziwei_json(...args)); }
  const year = Number(el('zwYear').value);
  if (!year || year < 1900 || year > 2100) { alert('填一个 1900–2100 的年份'); return; }
  const liu = JSON.parse(w.ziwei_liunian_json(...args, year));
  render(lastChart, liu);
  el('zwLiuText').textContent = `${liu.year} ${liu.year_ganzhi}年：流年命宫在${liu.soul_branch}宫（本命${liu.soul_palace_name}）· 流年四化：`
    + liu.mutagens.map(h => `${h.star}化${h.hua}（在${h.palace}）`).join('、');
};
}
