// 页面模块：今日一签。只负责本功能的页面交互，计算经 w（Rust 核心绑定）完成；
// 与其他功能模块不互相引用，共享物只从 ctx.js 取（低耦合约定）。
import { ctx } from './ctx.js?v=5.3.0';

export function initQian(w) {
const { Y, M, D } = ctx;
// 今日一签
  btnQian.onclick = () => {
  const q = JSON.parse(w.daily_qian_json(Y, M, D));
  ctx.state.lastQian = q;
  qGrade.textContent = `第 ${q.id} 签 · ${q.grade}`;
  qPoem.innerHTML = q.poem.join('<br>');
  qJie.textContent = '解签：' + q.jie;
  qianBefore.classList.add('hidden');
  qianAfter.classList.remove('hidden');
  qianAfter.classList.add('fade');
};
}
