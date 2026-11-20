// 页面模块：答案之书。只负责本功能的页面交互，计算经 w（Rust 核心绑定）完成；
// 与其他功能模块不互相引用，共享物只从 ctx.js 取（低耦合约定）。

export function initAnswers(w) {
// 答案之书
btnAns.onclick = () => {
  ansBox.textContent = w.book_answer(Math.random() * 2 ** 53);
  ansBox.classList.remove('hidden', 'fade');
  void ansBox.offsetWidth;
  ansBox.classList.add('fade');
  btnAns.textContent = '再 翻 一 次';
};
}
