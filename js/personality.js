// 页面模块：16 型人格（心智馆）。48 道原创题逐题作答，核心层计分。
// 本测试为通俗四维偏好测试，与 MBTI® 官方测评无关，结果仅供自我探索参考。
import { el } from './ctx.js?v=6.0.1';

export function initPersonality(w) {
  const bank = JSON.parse(w.personality_json());
  const dimNames = { EI: ['外向', '内向'], SN: ['实感', '直觉'], TF: ['理性', '感性'], JP: ['计划', '随性'] };
  let idx = 0;
  const ans = [];
  const intro = el('persIntro'), quiz = el('persQuiz'), result = el('persResult');

  function showQ() {
    const q = bank.questions[idx];
    el('persStep').textContent = `第 ${idx + 1} / 48 题`;
    el('persBar').style.width = ((idx + 1) / 48 * 100) + '%';
    el('persQ').textContent = q.q;
    el('persA').textContent = q.a.t;
    el('persB').textContent = q.b.t;
  }
  function finish() {
    quiz.classList.add('hidden');
    const r = JSON.parse(w.personality_score(ans.join('')));
    if (r.error) { result.innerHTML = `<p class="fineprint">${r.error}</p>`; result.classList.remove('hidden'); return; }
    const t = bank.types[r.type];
    const bars = r.dims.map(d => {
      const [ln, rn] = dimNames[d.dim];
      return `<div class="dimrow"><b>${d.left} ${ln} ${d.left_pct}%</b><div class="dimbar"><i style="width:${d.left_pct}%"></i></div><b>${d.right_pct}% ${rn} ${d.right}</b></div>`;
    }).join('');
    result.innerHTML = `
      <div class="perstype"><div class="code serif">${r.type}</div>
      <div class="pname">${t.name}</div><div class="ptag">${t.tagline}</div></div>
      ${bars}
      <p class="persdesc" style="margin-top:12px">${t.desc}</p>
      <div class="perscols"><div><h3>你的优势</h3><ul>${t.strengths.map(s => `<li>${s}</li>`).join('')}</ul></div>
      <div><h3>容易踩的坑</h3><ul>${t.blinds.map(s => `<li>${s}</li>`).join('')}</ul></div></div>
      <p class="persmotto">「${t.motto}」</p>
      <button class="btn serif" id="btnPersRedo">重新测试</button>`;
    result.classList.remove('hidden');
    result.classList.add('fade');
    el('btnPersRedo').onclick = () => {
      result.classList.add('hidden');
      intro.classList.remove('hidden');
    };
  }
  function pick(c) {
    ans[idx] = c;
    if (idx < 47) { idx++; showQ(); } else finish();
  }
  el('btnPersStart').onclick = () => {
    idx = 0; ans.length = 0;
    intro.classList.add('hidden');
    result.classList.add('hidden');
    quiz.classList.remove('hidden');
    showQ();
  };
  el('persA').onclick = () => pick('a');
  el('persB').onclick = () => pick('b');
}
