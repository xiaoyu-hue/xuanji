// 页面模块：大五人格（心智馆）。IPIP 50 题（公有领域条目，本站翻译），
// 五点量表逐题作答，核心层计分。科普性质，非临床评估。
import { el } from './ctx.js?v=6.1.0';

const SCALE = ['很不符合', '不太符合', '说不准', '比较符合', '非常符合'];

export function initBigfive(w) {
  const items = JSON.parse(w.bigfive_json());
  let idx = 0;
  const ans = [];
  const intro = el('bfIntro'), quiz = el('bfQuiz'), result = el('bfResult');

  function showQ() {
    const it = items[idx];
    el('bfStep').textContent = `第 ${idx + 1} / 50 题`;
    el('bfBar').style.width = ((idx + 1) / 50 * 100) + '%';
    el('bfQ').textContent = `「${it.zh}」`;
    el('bfOpts').innerHTML = SCALE.map((s, i) =>
      `<button class="optbtn" data-v="${i + 1}">${s}</button>`).join('');
    el('bfOpts').querySelectorAll('.optbtn').forEach(b => {
      b.onclick = () => {
        ans[idx] = b.dataset.v;
        if (idx < 49) { idx++; showQ(); } else finish();
      };
    });
  }
  function finish() {
    quiz.classList.add('hidden');
    const r = JSON.parse(w.bigfive_score(ans.join('')));
    if (r.error) { result.innerHTML = `<p class="fineprint">${r.error}</p>`; result.classList.remove('hidden'); return; }
    result.innerHTML = r.traits.map(t => `
      <div class="dimrow"><b>${t.name}</b><div class="dimbar"><i style="width:${(t.score - 10) / 40 * 100}%"></i></div><b>${t.score} · ${t.band}</b></div>
      <p class="persdesc">${t.text}</p>`).join('') +
      `<button class="btn serif" id="btnBfRedo">重新测试</button>`;
    result.classList.remove('hidden');
    result.classList.add('fade');
    el('btnBfRedo').onclick = () => {
      result.classList.add('hidden');
      intro.classList.remove('hidden');
    };
  }
  el('btnBfStart').onclick = () => {
    idx = 0; ans.length = 0;
    intro.classList.add('hidden');
    result.classList.add('hidden');
    quiz.classList.remove('hidden');
    showQ();
  };
}
