// 页面模块：生命灵数（西方馆）。出生日期算核心数字；计算经 w 完成。
import { el, ctx } from './ctx.js?v=6.0.1';

export function initLifepath(w) {
  el('lpYear').value = ctx.Y;
  el('btnLp').onclick = () => {
    const v = el('lpBirth').value;
    if (!v) return;
    const [y, m, d] = v.split('-').map(Number);
    const year = parseInt(el('lpYear').value, 10) || ctx.Y;
    const r = JSON.parse(w.lifepath_json(y, m, d, year));
    const block = (label, x) => `
      <div class="lpblock"><div class="lpnum serif">${x.n}</div>
      <div><b>${label} · ${x.name}</b><p>${x.trait_}</p><p class="fineprint">${x.tip}</p></div></div>`;
    el('lpOut').innerHTML =
      block('生命路径数', r.life_path) +
      block('生日数', r.birthday) +
      block('态度数', r.attitude) +
      block(`${r.target_year} 个人年数`, r.personal_year);
    el('lpOut').classList.remove('hidden');
  };
}
