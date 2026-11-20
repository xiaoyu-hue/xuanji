// 页面共享上下文：小工具、今日日期、跨功能共享状态。
// 所有功能模块只从这里取共享物，不互相 import（低耦合约定）。
export const el = id => document.getElementById(id);
export const pad = n => String(n).padStart(2, '0');

const now = new Date();
export const ctx = {
  Y: now.getFullYear(),
  M: now.getMonth() + 1,
  D: now.getDate(),
  week: ['日', '一', '二', '三', '四', '五', '六'][now.getDay()],
  pad,
  // 跨模块共享状态：黄历数据与最近一签（供分享图模块读取）
  state: { almData: null, almYMD: null, lastQian: null },
};
ctx.state.almYMD = [ctx.Y, ctx.M, ctx.D];
