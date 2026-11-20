// 页面模块：星座运势（西方馆）。按生日定星座，看今日运势；计算经 w 完成。
import { el, ctx } from './ctx.js?v=6.0.1';

export function initHoroscope(w) {
  const signs = ['白羊座', '金牛座', '双子座', '巨蟹座', '狮子座', '处女座', '天秤座', '天蝎座', '射手座', '摩羯座', '水瓶座', '双鱼座'];
  const chips = el('hsChips');
  signs.forEach((s, i) => {
    const b = document.createElement('button');
    b.className = 'zchip';
    b.textContent = s;
    b.onclick = () => show(i);
    chips.appendChild(b);
  });
  function bar(label, v) {
    return `<div class="dimrow"><b>${label}</b><div class="dimbar"><i style="width:${v}%"></i></div><b>${v}</b></div>`;
  }
  function show(i) {
    const f = JSON.parse(w.zodiac_fortune_json(i, ctx.Y, ctx.M, ctx.D));
    [...chips.children].forEach((c, k) => c.classList.toggle('on', k === i));
    el('hsOut').innerHTML = `
      <div class="perstype"><div class="code serif">${f.sign}</div>
      <div class="pname">${ctx.M} 月 ${ctx.D} 日 · 今日运势</div></div>
      ${bar('综合', f.overall)}${bar('爱情', f.love)}${bar('事业', f.career)}${bar('财运', f.wealth)}
      <p class="persdesc">幸运色 <b>${f.lucky_color}</b> · 幸运数字 <b>${f.lucky_number}</b></p>
      <p class="persdesc">${f.guide}</p>`;
    el('hsOut').classList.remove('hidden');
  }
  el('btnHs').onclick = () => {
    const v = el('hsBirth').value;
    if (!v) return;
    const [, m, d] = v.split('-').map(Number);
    show(w.zodiac_index(m, d));
  };
}
