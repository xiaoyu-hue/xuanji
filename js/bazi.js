// 页面模块：八字排盘。只负责本功能的页面交互，计算经 w（Rust 核心绑定）完成；
// 与其他功能模块不互相引用，共享物只从 ctx.js 取（低耦合约定）。
import { ctx } from './ctx.js?v=5.3.0';

export function initBazi(w) {
const { Y, M, D, pad } = ctx;
// 八字排盘
bzDate.value = `${Y}-${pad(M)}-${pad(D)}`;
let bzG = 1;
bzGender.querySelectorAll('button').forEach(b => b.onclick = () => {
  bzG = Number(b.dataset.g);
  bzGender.querySelectorAll('button').forEach(x => x.classList.toggle('on', x === b));
});
btnBazi.onclick = () => {
  if (!bzDate.value) return;
  const [y, m, d] = bzDate.value.split('-').map(Number);
  const [hh, mm] = (bzTime.value || '12:00').split(':').map(Number);
  const z = JSON.parse(w.bazi_json(y, m, d, hh, mm, bzG));
  bzPillars.innerHTML = z.pillars.map((p, i) => `
    <div class="pillar ${i === 2 ? 'day' : ''}">
      <div class="pname">${p.name}${i === 2 ? ' · 日主' : ''}</div>
      <div class="ss">${p.shi_shen_gan}</div>
      <div class="gzchar serif">${p.gan}<br>${p.zhi}</div>
      <div class="hide">藏 ${p.hide_gan.join(' ')}<br>${p.na_yin} · ${p.di_shi}</div>
      ${p.shensha && p.shensha.length ? `<div class="ssha">${p.shensha.join(' · ')}</div>` : ''}
    </div>`).join('');
  bzMeta.innerHTML = `公历 ${z.solar} · 农历${z.lunar_text}<br>日主 <em>${z.day_gan}（${z.day_wuxing}）</em> · 胎元 ${z.tai_yuan} · 命宫 ${z.ming_gong} · 身宫 ${z.shen_gong}`;
  const maxC = Math.max(...z.wuxing.map(w => w.count), 1);
  bzWx.innerHTML = z.wuxing.map(w =>
    `<div class="row"><label>${w.name}</label><div class="bar"><i style="width:${Math.round(w.count / maxC * 100)}%"></i></div><b>${w.count}</b></div>`).join('')
    + `<div class="lucky">${z.missing.length ? '五行缺 ' + z.missing.join('、') : '五行俱全'} · 最旺 ${z.strongest}</div>`;
  bzYun.innerHTML = `大运${z.forward ? '顺行' : '逆行'} · ${z.start_desc}`;
  bzDayun.innerHTML = z.da_yun.map(dy =>
    `<div class="dy"><b class="serif">${dy.ganzhi}</b><span>${dy.start_age}–${dy.end_age} 岁</span><span>${dy.start_year}–${dy.end_year}</span></div>`).join('');
  baziBox.classList.remove('hidden');
  baziBox.classList.remove('fade'); void baziBox.offsetWidth; baziBox.classList.add('fade');
};
}
