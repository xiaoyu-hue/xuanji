import init, { almanac_json, daily_qian_json, book_answer, zodiac_index, zodiac_name, fortune_json, bazi_json, personality_json, personality_score, daily_tarot_json, tarot_draw_json }
  from './pkg/xuanji_web.js?v=2.3.1';

const now = new Date();
const Y = now.getFullYear(), M = now.getMonth() + 1, D = now.getDate();
const week = ['日','一','二','三','四','五','六'][now.getDay()];

async function main(){
  await init('./pkg/xuanji_web_bg.wasm?v=2.3.1');
  document.getElementById('todayLine').textContent =
    `${Y} 年 ${M} 月 ${D} 日 · 星期${week}`;
  document.getElementById('loading').classList.add('hidden');
  document.getElementById('app').classList.remove('hidden');

  // 黄历（可查任意日期）
  const pad = n => String(n).padStart(2, '0');
  let almData = null, almYMD = [Y, M, D];
  const renderAlmanac = (y, m, d) => {
    almYMD = [y, m, d];
    const a = JSON.parse(almanac_json(y, m, d));
    almData = a;
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
  };
  datePick.value = `${Y}-${pad(M)}-${pad(D)}`;
  datePick.onchange = () => {
    if (!datePick.value) return;
    const [y, m, d] = datePick.value.split('-').map(Number);
    renderAlmanac(y, m, d);
  };
  btnToday.onclick = () => { datePick.value = `${Y}-${pad(M)}-${pad(D)}`; renderAlmanac(Y, M, D); };
  renderAlmanac(Y, M, D);

  // 今日一签
  let lastQian = null;
  btnQian.onclick = () => {
    const q = JSON.parse(daily_qian_json(Y, M, D));
    lastQian = q;
    qGrade.textContent = `第 ${q.id} 签 · ${q.grade}`;
    qPoem.innerHTML = q.poem.join('<br>');
    qJie.textContent = '解签：' + q.jie;
    qianBefore.classList.add('hidden');
    qianAfter.classList.remove('hidden');
    qianAfter.classList.add('fade');
  };

  // 答案之书
  btnAns.onclick = () => {
    ansBox.textContent = book_answer(Math.random() * 2 ** 53);
    ansBox.classList.remove('hidden', 'fade');
    void ansBox.offsetWidth;
    ansBox.classList.add('fade');
    btnAns.textContent = '再 翻 一 次';
  };

  // 星座运势
  let cur = zodiac_index(M, D);
  const renderSigns = () => {
    signGrid.innerHTML = '';
    for (let i = 0; i < 12; i++) {
      const b = document.createElement('button');
      b.textContent = zodiac_name(i);
      if (i === cur) b.classList.add('on');
      b.onclick = () => { cur = i; renderSigns(); renderFortune(); };
      signGrid.appendChild(b);
    }
  };
  const renderFortune = () => {
    const f = JSON.parse(fortune_json(cur, Y, M, D));
    const row = (label, v) =>
      `<div class="row"><label>${label}</label><div class="bar"><i style="width:${v}%"></i></div><b>${v}</b></div>`;
    fScores.innerHTML = row('整体', f.overall) + row('爱情', f.love) + row('事业', f.career) + row('财运', f.wealth);
    fLucky.innerHTML = `幸运色 <em>${f.lucky_color}</em> · 幸运数字 <em>${f.lucky_number}</em><br>${f.guide}`;
    fortuneBox.classList.remove('hidden');
    fortuneBox.classList.add('fade');
  };
  renderSigns(); renderFortune();

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
    const z = JSON.parse(bazi_json(y, m, d, hh, mm, bzG));
    bzPillars.innerHTML = z.pillars.map((p, i) => `
      <div class="pillar ${i === 2 ? 'day' : ''}">
        <div class="pname">${p.name}${i === 2 ? ' · 日主' : ''}</div>
        <div class="ss">${p.shi_shen_gan}</div>
        <div class="gzchar serif">${p.gan}<br>${p.zhi}</div>
        <div class="hide">藏 ${p.hide_gan.join(' ')}<br>${p.na_yin} · ${p.di_shi}</div>
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
    if (!almData) return;
    const a = almData, [y, m, d] = almYMD;
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
    if (!lastQian) return;
    const q = lastQian;
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

  // 16 型人格
  const pBank = JSON.parse(personality_json());
  const dimNames = { EI: ['外向', '内向'], SN: ['实感', '直觉'], TF: ['理性', '感性'], JP: ['计划', '随性'] };
  let pIdx = 0, pAns = [], lastPers = null;
  const persIntro = document.getElementById('persIntro'), persQuiz = document.getElementById('persQuiz'),
        persResult = document.getElementById('persResult'), sharePers = document.getElementById('sharePers');
  const showPersQ = () => {
    const q = pBank.questions[pIdx];
    document.getElementById('persStep').textContent = `第 ${pIdx + 1} / 48 题`;
    document.getElementById('persBar').style.width = ((pIdx + 1) / 48 * 100) + '%';
    document.getElementById('persQ').textContent = q.q;
    document.getElementById('persA').textContent = q.a.t;
    document.getElementById('persB').textContent = q.b.t;
  };
  const finishPers = () => {
    persQuiz.classList.add('hidden');
    const r = JSON.parse(personality_score(pAns.join('')));
    const t = pBank.types[r.type];
    lastPers = { r, t };
    const bars = r.dims.map(d => {
      const [ln, rn] = dimNames[d.dim];
      return `<div class="dimrow"><b>${d.left} ${ln} ${d.left_pct}%</b><div class="dimbar"><i style="width:${d.left_pct}%"></i></div><b>${d.right_pct}% ${rn} ${d.right}</b></div>`;
    }).join('');
    persResult.innerHTML = `
      <div class="perstype"><div class="code serif">${r.type}</div>
      <div class="pname">${t.name}</div><div class="ptag">${t.tagline}</div></div>
      ${bars}
      <p class="persdesc" style="margin-top:12px">${t.desc}</p>
      <div class="perscols"><div><h3>你的优势</h3><ul>${t.strengths.map(s => `<li>${s}</li>`).join('')}</ul></div>
      <div><h3>容易踩的坑</h3><ul>${t.blinds.map(s => `<li>${s}</li>`).join('')}</ul></div></div>
      <p class="persmotto">「${t.motto}」</p>
      <div class="persbtns"><button class="btn serif" id="btnPersRedo">重新测试</button>
      <button class="btn serif" id="btnPersShare">生成分享图</button></div>`;
    persResult.classList.remove('hidden'); persResult.classList.add('fade');
    document.getElementById('btnPersRedo').onclick = () => {
      persResult.classList.add('hidden'); sharePers.classList.add('hidden');
      persIntro.classList.remove('hidden');
    };
    document.getElementById('btnPersShare').onclick = () => {
      const { r, t } = lastPers;
      const [c, x] = newCard(1120);
      x.fillStyle = '#a63d2f'; x.font = `700 92px ${SERIF}`;
      x.fillText(r.type, 375, 268);
      x.fillStyle = '#2e2a26'; x.font = `600 46px ${SERIF}`;
      x.fillText(t.name, 375, 344);
      x.fillStyle = '#6f6659'; x.font = '27px sans-serif';
      x.fillText(t.tagline, 375, 392);
      let yy = 486; x.font = '28px sans-serif'; x.fillStyle = '#2e2a26';
      for (const d of r.dims) {
        const [ln, rn] = dimNames[d.dim];
        x.fillText(`${d.left} ${ln} ${d.left_pct}%  ·  ${d.right_pct}% ${rn} ${d.right}`, 375, yy); yy += 52;
      }
      yy += 22; x.font = `600 32px ${SERIF}`; x.fillStyle = '#a63d2f';
      x.fillText(`「${t.motto}」`, 375, yy);
      x.fillStyle = '#6f6659'; x.font = '23px sans-serif';
      x.fillText('玄机 · 16 型人格测试（本站原创题库）', 375, yy + 52);
      foot(x, 1120);
      showShare(c, sharePers, 'xuanji-personality.png');
    };
  };
  const pickPers = (c) => {
    pAns[pIdx] = c;
    if (pIdx < 47) { pIdx++; showPersQ(); } else finishPers();
  };
  document.getElementById('btnPersStart').onclick = () => {
    pIdx = 0; pAns = [];
    persIntro.classList.add('hidden'); persResult.classList.add('hidden'); sharePers.classList.add('hidden');
    persQuiz.classList.remove('hidden'); showPersQ();
  };
  document.getElementById('persA').onclick = () => pickPers('a');
  document.getElementById('persB').onclick = () => pickPers('b');
  document.getElementById('persBack').onclick = () => { if (pIdx > 0) { pIdx--; showPersQ(); } };

  // 今日塔罗
  const ROMAN = ['0','I','II','III','IV','V','VI','VII','VIII','IX','X','XI','XII','XIII','XIV','XV','XVI','XVII','XVIII','XIX','XX','XXI'];
  let lastTarot = null;
  const tarotIntro = document.getElementById('tarotIntro'), tarotBox = document.getElementById('tarotBox'),
        shareTarot = document.getElementById('shareTarot');
  const showTarot = (t) => {
    lastTarot = t;
    tarotIntro.classList.add('hidden'); shareTarot.classList.add('hidden');
    const face = document.getElementById('tarotFace');
    face.classList.remove('rev');
    document.getElementById('tarotNum').textContent = ROMAN[t.num];
    document.getElementById('tarotName').textContent = t.name;
    document.getElementById('tarotEn').textContent = t.en;
    document.getElementById('tarotRevTag').classList.toggle('hidden', !t.reversed);
    document.getElementById('tarotKw').textContent = t.kw;
    document.getElementById('tarotMean').textContent = (t.reversed ? '【逆位】' : '【正位】') + t.meaning;
    tarotBox.classList.remove('hidden'); tarotBox.classList.add('fade');
    if (t.reversed) requestAnimationFrame(() => requestAnimationFrame(() => face.classList.add('rev')));
  };
  document.getElementById('btnTarot').onclick = () => showTarot(JSON.parse(daily_tarot_json(Y, M, D)));
  document.getElementById('btnTarotRand').onclick = () => showTarot(JSON.parse(tarot_draw_json(Math.random() * 1e18)));
  document.getElementById('btnTarotShare').onclick = () => {
    if (!lastTarot) return;
    const t = lastTarot;
    const [c, x] = newCard(1060);
    x.fillStyle = '#b8862f'; x.font = '26px sans-serif';
    x.fillText(`今 日 塔 罗 · ${ROMAN[t.num]}`, 375, 232);
    x.fillStyle = '#2e2a26'; x.font = `700 84px ${SERIF}`;
    x.fillText(t.name, 375, 348);
    x.fillStyle = '#6f6659'; x.font = '25px sans-serif';
    x.fillText(t.en + (t.reversed ? ' · 逆位' : ' · 正位'), 375, 398);
    x.fillStyle = '#b8862f'; x.font = '28px sans-serif';
    x.fillText(t.kw, 375, 472);
    x.fillStyle = '#2e2a26'; x.font = '29px sans-serif';
    const ml = wrap(x, t.meaning, 590);
    ml.forEach((ln, i) => x.fillText(ln, 375, 556 + i * 50));
    x.fillStyle = '#6f6659'; x.font = '23px sans-serif';
    x.fillText(`${Y} 年 ${M} 月 ${D} 日`, 375, 556 + ml.length * 50 + 42);
    foot(x, 1060);
    showShare(c, shareTarot, 'xuanji-tarot.png');
  };
}

main().catch(e => {
  document.getElementById('loading').textContent = '黄历没翻开，请刷新重试';
  console.error(e);
});
