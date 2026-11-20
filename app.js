import init, { almanac_json, daily_qian_json, book_answer, bazi_json }
  from './pkg/xuanji_web.js?v=3.0.0';

const now = new Date();
const Y = now.getFullYear(), M = now.getMonth() + 1, D = now.getDate();
const week = ['日','一','二','三','四','五','六'][now.getDay()];

async function main(){
  await init('./pkg/xuanji_web_bg.wasm?v=3.0.0');
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

}

main().catch(e => {
  document.getElementById('loading').textContent = '黄历没翻开，请刷新重试';
  console.error(e);
});
