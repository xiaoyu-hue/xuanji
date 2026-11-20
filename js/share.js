// 页面模块：分享图。只负责本功能的页面交互，计算经 w（Rust 核心绑定）完成；
// 与其他功能模块不互相引用，共享物只从 ctx.js 取（低耦合约定）。
import { ctx } from './ctx.js?v=5.1.0';

export function initShare(w) {
const { Y, M, D } = ctx;
// 分享图（在浏览器本地用画布生成，不经服务器）
const SERIF = '"Noto Serif SC","Songti SC","STSong",serif';
const wrap = (ctx, text, maxW) => {
  const lines = []; let cur = '';
  for (const ch of text) {
    if (ctx.measureText(cur + ch).width > maxW) { lines.push(cur); cur = ch; } else cur += ch;
  }
  if (cur) lines.push(cur);
  return lines;
};
const newCard = (h) => {
  const c = document.createElement('canvas'); c.width = 750; c.height = h;
  const x = c.getContext('2d');
  x.fillStyle = '#f7f1e5'; x.fillRect(0, 0, 750, h);
  x.strokeStyle = '#a63d2f'; x.lineWidth = 3; x.strokeRect(24, 24, 702, h - 48);
  x.strokeStyle = '#e3d7bf'; x.lineWidth = 1; x.strokeRect(36, 36, 678, h - 72);
  x.textAlign = 'center';
  x.fillStyle = '#a63d2f'; x.font = `600 44px ${SERIF}`;
  x.fillText('玄 机', 375, 108);
  x.fillStyle = '#6f6659'; x.font = '24px sans-serif';
  x.fillText('民 俗 · 娱 乐', 375, 148);
  return [c, x];
};
const foot = (x, h) => {
  x.fillStyle = '#6f6659'; x.font = '23px sans-serif'; x.textAlign = 'center';
  x.fillText('xiaoyu-hue.github.io/xuanji', 375, h - 58);
};
const showShare = (canvas, box, name) => {
  const url = canvas.toDataURL('image/png');
  box.innerHTML = `<img src="${url}" alt="分享图"><br><a href="${url}" download="${name}">点此下载图片</a><p>手机上也可以长按图片保存</p>`;
  box.classList.remove('hidden'); box.classList.add('fade');
};
btnShareAlm.onclick = () => {
  if (!ctx.state.almData) return;
  const a = ctx.state.almData, [y, m, d] = ctx.state.almYMD;
  const [c, x] = newCard(1060);
  x.fillStyle = '#2e2a26'; x.font = `600 54px ${SERIF}`;
  x.fillText(a.lunar_text, 375, 238);
  x.fillStyle = '#6f6659'; x.font = '27px sans-serif';
  x.fillText(`${y} 年 ${m} 月 ${d} 日 · ${a.year_ganzhi}年 ${a.month_ganzhi}月 ${a.day_ganzhi}日 · 属${a.shengxiao}`, 375, 292);
  let yy = 372;
  x.textAlign = 'left'; x.font = '28px sans-serif';
  x.fillStyle = '#3f6f5f'; x.fillText('宜', 84, yy);
  x.fillStyle = '#2e2a26';
  for (const ln of wrap(x, a.yi.join(' · '), 560)) { x.fillText(ln, 140, yy); yy += 42; }
  yy += 18;
  x.fillStyle = '#a63d2f'; x.fillText('忌', 84, yy);
  x.fillStyle = '#2e2a26';
  for (const ln of wrap(x, a.ji.join(' · '), 560)) { x.fillText(ln, 140, yy); yy += 42; }
  yy += 26;
  x.font = '27px sans-serif'; x.fillStyle = '#2e2a26'; x.textAlign = 'center';
  const lines = [
    `值神 ${a.tianshen}（${a.tianshen_luck}） · 星宿 ${a.xiu}（${a.xiu_luck}）`,
    `冲煞 冲${a.chong} · 煞${a.sha}`,
    `财神位 ${a.cai} · 喜神位 ${a.xi} · 福神位 ${a.fu}`,
    `节气 ${a.prev_jieqi} → ${a.next_jieqi}`,
  ];
  for (const ln of lines) { x.fillText(ln, 375, yy); yy += 48; }
  foot(x, 1060);
  showShare(c, shareAlm, 'xuanji-almanac.png');
};
btnShareQian.onclick = () => {
  if (!ctx.state.lastQian) return;
  const q = ctx.state.lastQian;
  const [c, x] = newCard(1000);
  x.fillStyle = '#b8862f'; x.font = `600 32px ${SERIF}`;
  x.fillText(`第 ${q.id} 签 · ${q.grade}`, 375, 240);
  x.fillStyle = '#2e2a26'; x.font = `600 46px ${SERIF}`;
  q.poem.forEach((ln, i) => x.fillText(ln, 375, 350 + i * 78));
  x.fillStyle = '#6f6659'; x.font = '27px sans-serif';
  const jl = wrap(x, '解签：' + q.jie, 580);
  jl.forEach((ln, i) => x.fillText(ln, 375, 730 + i * 44));
  x.font = '24px sans-serif';
  x.fillText(`${Y} 年 ${M} 月 ${D} 日 · 今日一签`, 375, 730 + jl.length * 44 + 48);
  foot(x, 1000);
  showShare(c, shareQian, 'xuanji-qian.png');
};
}
