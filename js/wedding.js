// 页面模块：万事择日（原婚嫁择日升级）。只负责本功能的页面交互，计算经 w（Rust 核心绑定）完成；
// 与其他功能模块不互相引用，共享物只从 ctx.js 取（低耦合约定）。
import { el } from './ctx.js?v=6.0.1';

export function initWedding(w) {
// 万事择日：事项可选，默认嫁娶（与原婚嫁择日行为一致）
const ACTS = ['嫁娶', '出行', '移徙', '入宅', '开市', '交易', '纳财', '立券', '订盟', '纳采', '会亲友', '祈福', '祭祀', '动土', '修造', '安床', '栽种', '盖屋'];
el('wedAct').innerHTML = ACTS.map(a => `<option value="${a}">宜${a}</option>`).join('');
const ZOD = ['鼠', '牛', '虎', '兔', '龙', '蛇', '马', '羊', '猴', '鸡', '狗', '猪'];
const fillZod = (sel, label) => { sel.innerHTML = `<option value="">${label}</option>` + ZOD.map(z => `<option value="${z}">${z}</option>`).join(''); };
fillZod(el('wedGroom'), '本人生肖（不限）');
fillZod(el('wedBride'), '家人生肖（不限）');
const fmtD = dt => dt.toISOString().slice(0, 10);
el('wedFrom').value = fmtD(new Date());
el('wedTo').value = fmtD(new Date(Date.now() + 90 * 864e5));
// 分段查：一天一天翻黄历很吃算力，一次算完会把页面冻住；
// 每 3 天让出一次主线程并报进度，页面始终能点能滑。
el('btnWed').onclick = async () => {
  const f = el('wedFrom').value, t = el('wedTo').value;
  if (!f || !t) { alert('先选好起止日期'); return; }
  const [y1, m1, d1] = f.split('-').map(Number);
  const [y2, m2, d2] = t.split('-').map(Number);
  const startUtc = Date.UTC(y1, m1 - 1, d1);
  let endUtc = Date.UTC(y2, m2 - 1, d2);
  if (endUtc < startUtc) { alert('结束日期要晚于开始日期'); return; }
  if (endUtc - startUtc > 400 * 864e5) endUtc = startUtc + 400 * 864e5;
  const totalDays = Math.round((endUtc - startUtc) / 864e5) + 1;
  const groom = el('wedGroom').value, bride = el('wedBride').value;
  const act = el('wedAct').value;
  const btn = el('btnWed');
  btn.disabled = true; btn.textContent = '正在翻黄历…';
  el('wedBox').classList.remove('hidden');
  el('wedList').innerHTML = '';
  el('wedCount').textContent = `正在翻黄历…已查 0/${totalDays} 天`;
  const render = (list) => {
    el('wedList').innerHTML = list.map(x =>
      `<div class="wday"><b>${x.solar} ${x.weekday}</b><span>${x.lunar_text}</span><span class="wl">${x.day_ganzhi}日 · 冲${x.chong}</span></div>`).join('');
  };
  let all = [];
  try {
    for (let cur = startUtc; cur <= endUtc; cur += 3 * 864e5) {
      const ce = Math.min(cur + 2 * 864e5, endUtc);
      const cd = new Date(cur), ed = new Date(ce);
      const part = JSON.parse(w.day_search_json(
        cd.getUTCFullYear(), cd.getUTCMonth() + 1, cd.getUTCDate(),
        ed.getUTCFullYear(), ed.getUTCMonth() + 1, ed.getUTCDate(), act, groom, bride));
      all = all.concat(part);
      render(all);
      const done = Math.round((ce - startUtc) / 864e5) + 1;
      el('wedCount').textContent = done < totalDays
        ? `正在翻黄历…已查 ${done}/${totalDays} 天，先找到 ${all.length} 个好日子`
        : (all.length ? `共 ${all.length} 个宜${act}的好日子` : `这段时间里没有宜${act}的好日子，换个区间试试`);
      await new Promise(r => setTimeout(r, 10));
    }
  } finally {
    btn.disabled = false; btn.textContent = '查 吉 日';
  }
};
}
