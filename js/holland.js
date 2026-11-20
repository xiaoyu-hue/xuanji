// 页面模块：霍兰德职业兴趣（心智馆）。48 道原创题，五点量表，核心层计分。
import { el } from './ctx.js?v=6.1.0';

const SCALE = ['完全没兴趣', '不太感兴趣', '一般', '比较感兴趣', '非常感兴趣'];

export function initHolland(w) {
  const bank = JSON.parse(w.holland_json());
  const items = bank.items;
  let idx = 0;
  const ans = [];
  const intro = el('hlIntro'), quiz = el('hlQuiz'), result = el('hlResult');

  function showQ() {
    const it = items[idx];
    el('hlStep').textContent = `第 ${idx + 1} / 48 题`;
    el('hlBar').style.width = ((idx + 1) / 48 * 100) + '%';
    el('hlQ').textContent = it.q;
    el('hlOpts').innerHTML = SCALE.map((s, i) =>
      `<button class="optbtn" data-v="${i + 1}">${s}</button>`).join('');
    el('hlOpts').querySelectorAll('.optbtn').forEach(b => {
      b.onclick = () => {
        ans[idx] = b.dataset.v;
        if (idx < 47) { idx++; showQ(); } else finish();
      };
    });
  }
  function finish() {
    quiz.classList.add('hidden');
    const r = JSON.parse(w.holland_score(ans.join('')));
    if (r.error) { result.innerHTML = `<p class="fineprint">${r.error}</p>`; result.classList.remove('hidden'); return; }
    const top = r.types[0];
    result.innerHTML = `
      <div class="perstype"><div class="code serif">${r.code}</div>
      <div class="pname">你的兴趣代码</div></div>
      ${r.types.map(t => `<div class="dimrow"><b>${t.key} ${t.name}</b><div class="dimbar"><i style="width:${t.score / 40 * 100}%"></i></div><b>${t.score}</b></div>`).join('')}
      <p class="persdesc"><b>最突出的 ${top.name}</b>：${top.text}</p>
      <p class="persdesc">常对口的职业方向：${top.jobs}。</p>
      <p class="fineprint">兴趣代码按前三名组合看，比如 ${r.code} 型可在职业库里按这三个字母检索对口职业；兴趣会随经历变化，隔段时间可重测。</p>
      <button class="btn serif" id="btnHlRedo">重新测试</button>`;
    result.classList.remove('hidden');
    result.classList.add('fade');
    el('btnHlRedo').onclick = () => {
      result.classList.add('hidden');
      intro.classList.remove('hidden');
    };
  }
  el('btnHlStart').onclick = () => {
    idx = 0; ans.length = 0;
    intro.classList.add('hidden');
    result.classList.add('hidden');
    quiz.classList.remove('hidden');
    showQ();
  };
}
