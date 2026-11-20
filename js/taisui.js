// 页面模块：犯太岁。只负责本功能的页面交互，计算经 w（Rust 核心绑定）完成；
// 与其他功能模块不互相引用，共享物只从 ctx.js 取（低耦合约定）。
import { ctx, el } from './ctx.js?v=6.1.0';

export function initTaisui(w) {
  const ZOD = ['鼠', '牛', '虎', '兔', '龙', '蛇', '马', '羊', '猴', '鸡', '狗', '猪'];
  el('tsZodiac').innerHTML = '<option value="">我的生肖（可不选）</option>'
    + ZOD.map(z => `<option value="${z}">属${z}</option>`).join('');
  el('tsYear').value = ctx.Y + 1;
  const run = () => {
    const yr = Number(el('tsYear').value) || ctx.Y + 1;
    const zod = el('tsZodiac').value;
    const r = JSON.parse(w.taisui_json(yr, zod));
    el('tsBox').classList.remove('hidden');
    el('tsHead').textContent = `${r.year} 年 · ${r.year_ganzhi}（${r.year_zodiac}年）`;
    el('tsMine').textContent = !zod
      ? '这一年犯太岁的生肖'
      : (r.mine.length ? `属${zod} · ${r.mine.join(' · ')}` : `属${zod} · 不犯太岁`);
    el('tsText').textContent = r.mine_text;
    el('tsList').innerHTML = r.hits.map(h =>
      `<div class="wday"><b>属${h.zodiac}</b><span class="wl">${h.kinds.join(' · ')}</span></div>`).join('');
  };
  el('btnTaisui').onclick = run;
  run();
}
