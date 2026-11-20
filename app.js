import init, { almanac_json, daily_qian_json, book_answer, bazi_json }
  from './pkg/xuanji_web.js?v=3.0.0';

const now = new Date();
const Y = now.getFullYear(), M = now.getMonth() + 1, D = now.getDate();
const week = ['日','一','二','三','四','五','六'][now.getDay()];

async function main(){
  await init('./pkg/xuanji_web_bg.wasm?v=4.0.0');
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

// 婚嫁择日
  const ZOD = ['鼠', '牛', '虎', '兔', '龙', '蛇', '马', '羊', '猴', '鸡', '狗', '猪'];
  const fillZod = (sel, label) => { sel.innerHTML = `<option value="">${label}</option>` + ZOD.map(z => `<option value="${z}">${z}</option>`).join(''); };
  fillZod(el('wedGroom'), '新郎生肖（不限）');
  fillZod(el('wedBride'), '新娘生肖（不限）');
  const fmtD = dt => dt.toISOString().slice(0, 10);
  el('wedFrom').value = fmtD(new Date());
  el('wedTo').value = fmtD(new Date(Date.now() + 180 * 864e5));
  el('btnWed').onclick = () => {
    const f = el('wedFrom').value, t = el('wedTo').value;
    if (!f || !t) { alert('先选好起止日期'); return; }
    const [y1, m1, d1] = f.split('-').map(Number);
    const [y2, m2, d2] = t.split('-').map(Number);
    const days = JSON.parse(wedding_json(y1, m1, d1, y2, m2, d2, el('wedGroom').value, el('wedBride').value));
    el('wedCount').textContent = days.length ? `共 ${days.length} 个宜嫁娶的好日子` : '这段时间里没有合适的嫁娶吉日，换个区间试试';
    el('wedList').innerHTML = days.map(x =>
      `<div class="wday"><b>${x.solar} ${x.weekday}</b><span>${x.lunar_text}</span><span class="wl">${x.day_ganzhi}日 · 冲${x.chong}</span></div>`).join('');
    el('wedBox').classList.remove('hidden');
  };

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

// 称骨
  el('btnBone').onclick = () => {
    const dv = el('cgDate').value, tv = el('cgTime').value || '12:00';
    if (!dv) { alert('先选出生日期'); return; }
    const [y, m, d] = dv.split('-').map(Number);
    const [hh, mm] = tv.split(':').map(Number);
    const b = JSON.parse(bone_json(y, m, d, hh, mm));
    const fmtW = w => w >= 10 ? `${Math.floor(w / 10)}两${w % 10 ? w % 10 + '钱' : ''}` : `${w}钱`;
    el('boneMeta').innerHTML = `农历${b.lunar_text} · ${b.year_ganzhi}年 · ${b.hour_zhi}`;
    el('boneParts').innerHTML = [['年骨', b.year_w], ['月骨', b.month_w], ['日骨', b.day_w], ['时骨', b.hour_w]].map(([n, w]) =>
      `<div class="row"><label>${n}</label><div class="bar"><i style="width:${Math.round(w / 19 * 100)}%"></i></div><b>${fmtW(w)}</b></div>`).join('');
    el('boneTotal').textContent = `总骨重 ${b.total_text}`;
    el('boneVerse').textContent = b.verse;
    el('boneBox').classList.remove('hidden');
  };

  // 合婚
  el('btnHehun').onclick = () => {
    const ga = (id) => el(id).value, tm = (id) => (el(id).value || '12:00').split(':').map(Number);
    if (!ga('hhADate') || !ga('hhBDate')) { alert('先把两个人的出生日期选好'); return; }
    const [ay, am, ad] = ga('hhADate').split('-').map(Number);
    const [by, bm, bd] = ga('hhBDate').split('-').map(Number);
    const [ah, amin] = tm('hhATime'), [bh, bmin] = tm('hhBTime');
    const r = JSON.parse(hehun_json(ay, am, ad, ah, amin, by, bm, bd, bh, bmin));
    el('hehunScore').textContent = `${r.score} 分 · ${r.grade}`;
    el('hehunFactors').innerHTML = r.factors.map(f =>
      `<div class="factor"><div class="fh"><b>${f.name}：${f.result}</b><span class="delta ${f.delta >= 0 ? 'up' : 'down'}">${f.delta >= 0 ? '+' : ''}${f.delta}</span></div><p>${f.note}</p></div>`).join('');
    el('hehunBox').classList.remove('hidden');
  };

  // 解梦
  const dreamData = JSON.parse(dreams_json());
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

  // 紫微斗数
  let zwGender = 1;
  el('zwGender').querySelectorAll('button').forEach(b => b.onclick = () => {
    zwGender = Number(b.dataset.g);
    el('zwGender').querySelectorAll('button').forEach(x => x.classList.toggle('on', x === b));
  });
  const ZWPOS = { '巳': [1, 1], '午': [1, 2], '未': [1, 3], '申': [1, 4], '辰': [2, 1], '酉': [2, 4], '卯': [3, 1], '戌': [3, 4], '寅': [4, 1], '丑': [4, 2], '子': [4, 3], '亥': [4, 4] };
  el('btnZiwei').onclick = () => {
    const dv = el('zwDate').value, tv = el('zwTime').value || '12:00';
    if (!dv) { alert('先选出生日期'); return; }
    const [y, m, d] = dv.split('-').map(Number);
    const [hh, mm] = tv.split(':').map(Number);
    const c = JSON.parse(ziwei_json(y, m, d, hh, mm, zwGender));
    const huaCls = { '禄': 'lu', '权': 'quan', '科': 'ke', '忌': 'ji' };
    let html = `<div class="zwcenter"><span class="big serif">玄 机 命 盘</span><span>农历${c.lunar_text}</span><span>${c.year_ganzhi}年 · 命宫${c.soul_ganzhi} · 身宫${c.body_ganzhi}</span><span>${c.ju} · 紫微在${c.ziwei_branch}宫</span></div>`;
    for (const p of c.palaces) {
      const [r, col] = ZWPOS[p.branch];
      const stars = p.stars.map(st => st.kind === '主星'
        ? `<span class="maj serif">${st.name}${st.brightness ? `<sup>${st.brightness}</sup>` : ''}${st.hua ? `<span class="hua ${huaCls[st.hua]}">${st.hua}</span>` : ''}</span>`
        : `<span class="${st.kind === '煞星' ? 'sha' : 'min'}">${st.name}${st.brightness ? `<sup>${st.brightness}</sup>` : ''}${st.hua ? `<span class="hua ${huaCls[st.hua]}">${st.hua}</span>` : ''}</span>`).join('');
      html += `<div class="zwcell ${p.is_soul ? 'soul' : ''}" style="grid-row:${r};grid-column:${col}"><div class="zh"><span class="gz">${p.ganzhi}</span><span>${p.branch}宫</span></div><div class="pname">${p.name}${p.is_body ? '<span class="bodytag">身宫</span>' : ''}</div><div class="stars">${stars || '<span class="min">（空宫）</span>'}</div><div class="dx">大限 ${p.daxian} 岁</div></div>`;
    }
    el('zwGrid').innerHTML = html;
    el('zwBox').classList.remove('hidden');
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

}

main().catch(e => {
  document.getElementById('loading').textContent = '黄历没翻开，请刷新重试';
  console.error(e);
});
