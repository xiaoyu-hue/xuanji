// 页面模块：今日黄历。只负责本功能的页面交互，计算经 w（Rust 核心绑定）完成；
// 与其他功能模块不互相引用，共享物只从 ctx.js 取（低耦合约定）。
import { ctx } from './ctx.js?v=5.4.0';

export function initAlmanac(w) {
const { Y, M, D, pad } = ctx;
// 黄历（可查任意日期）
    const renderAlmanac = (y, m, d) => {
  ctx.state.almYMD = [y, m, d];
  const a = JSON.parse(w.almanac_json(y, m, d));
  ctx.state.almData = a;
  const isToday = (y === Y && m === M && d === D);
  almTitle.textContent = isToday ? '今日黄历' : '黄历查询';
  almSolar.textContent = a.solar.replaceAll('-', ' / ');
  almLunar.textContent = a.lunar_text;
  almGz.textContent = `${a.year_ganzhi}年 · ${a.month_ganzhi}月 · ${a.day_ganzhi}日 · 属${a.shengxiao}`;
  almYi.textContent = a.yi.join(' · ');
  almJi.textContent = a.ji.join(' · ');
  const rows = [
    ['值神', `${a.tianshen}（${a.tianshen_luck}）`], ['星宿', `${a.xiu}（${a.xiu_luck}）`],
    ['冲煞', `冲${a.chong} · 煞${a.sha}`], ['节气', `${a.prev_jieqi} → ${a.next_jieqi}`],
    ['财神位', a.cai], ['喜神位', a.xi], ['福神位', a.fu], ['星座', a.xingzuo + '座'],
  ];
  almGrid.innerHTML = rows.map(([k,v]) => `<div><span>${k}</span><b>${v}</b></div>`).join('');
  almPengzu.textContent = `彭祖百忌：${a.pengzu_gan} · ${a.pengzu_zhi}`;
  const hrs = JSON.parse(w.day_hours_json(y, m, d));
  almHours.innerHTML = hrs.map(h =>
    `<div class="hchip${h.luck === '凶' ? ' bad' : ''}"><b>${h.zhi}时</b><span>${h.range}</span><span>${h.ganzhi} · ${h.tianshen} · ${h.luck}</span></div>`).join('');
};
datePick.value = `${Y}-${pad(M)}-${pad(D)}`;
datePick.onchange = () => {
  if (!datePick.value) return;
  const [y, m, d] = datePick.value.split('-').map(Number);
  renderAlmanac(y, m, d);
};
btnToday.onclick = () => { datePick.value = `${Y}-${pad(M)}-${pad(D)}`; renderAlmanac(Y, M, D); };
renderAlmanac(Y, M, D);
}
