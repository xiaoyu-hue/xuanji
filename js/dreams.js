// 页面模块：解梦。只负责本功能的页面交互，计算经 w（Rust 核心绑定）完成；
// 与其他功能模块不互相引用，共享物只从 ctx.js 取（低耦合约定）。
import { el } from './ctx.js?v=5.0.1';

export function initDreams(w) {
// 解梦
const dreamData = JSON.parse(w.dreams_json());
let dreamCat = '全部';
const renderDreams = () => {
  const q = el('dreamQ').value.trim();
  const list = dreamData.dreams.filter(x =>
    (dreamCat === '全部' || x.cat === dreamCat) &&
    (!q || x.kw.includes(q) || x.title.includes(q) || x.text.includes(q)));
  el('dreamList').innerHTML = (list.length ? list.slice(0, 30).map(x =>
    `<div class="dream"><div class="dh">${x.title}<span class="luck ${x.luck === '偏凶' ? 'bad' : ''}">${x.luck}</span></div><p>${x.text}</p></div>`).join('')
    : '<p class="fineprint">没收录这个梦，换个关键词试试（如：水、蛇、飞）</p>') +
    (list.length > 30 ? `<p class="fineprint">共 ${list.length} 条，先显示前 30 条，输入关键词可缩小范围</p>` : '');
};
el('dreamCats').innerHTML = ['全部', ...dreamData.cats].map(c =>
  `<button data-c="${c}" class="${c === '全部' ? 'on' : ''}">${c}</button>`).join('');
el('dreamCats').querySelectorAll('button').forEach(b => b.onclick = () => {
  dreamCat = b.dataset.c;
  el('dreamCats').querySelectorAll('button').forEach(x => x.classList.toggle('on', x === b));
  renderDreams();
});
el('dreamQ').addEventListener('input', renderDreams);
renderDreams();
}
