// 页面模块：西洋星盘（西方馆）。本命盘：行星落座＋上升/中天＋等宫制
// 十二宫＋主要相位，星盘轮图由本模块以 SVG 绘制；计算经 w 完成。
import { el } from './ctx.js?v=6.1.0';

const CITIES = [
  ['北京', 116.41, 39.90], ['上海', 121.47, 31.23], ['广州', 113.26, 23.13],
  ['深圳', 114.06, 22.55], ['成都', 104.07, 30.67], ['西安', 108.94, 34.34],
  ['哈尔滨', 126.53, 45.80], ['乌鲁木齐', 87.62, 43.79], ['拉萨', 91.14, 29.65],
  ['香港', 114.17, 22.32], ['台北', 121.56, 25.03],
];

export function initNatal(w) {
  const sel = el('ntCity');
  sel.innerHTML = CITIES.map((c, i) => `<option value="${i}">${c[0]}</option>`).join('') +
    '<option value="-1">自定义经纬度</option>';
  const applyCity = () => {
    const i = parseInt(sel.value, 10);
    if (i >= 0) {
      el('ntLon').value = CITIES[i][1];
      el('ntLat').value = CITIES[i][2];
    }
  };
  sel.onchange = applyCity;
  applyCity();

  el('btnNt').onclick = () => {
    const dv = el('ntBirth').value;
    const tv = el('ntTime').value || '12:00';
    if (!dv) return;
    const [y, m, d] = dv.split('-').map(Number);
    const [hh, mm] = tv.split(':').map(Number);
    const lon = parseFloat(el('ntLon').value);
    const lat = parseFloat(el('ntLat').value);
    if (Number.isNaN(lon) || Number.isNaN(lat)) return;
    const c = JSON.parse(w.natal_json(y, m, d, hh, mm, lon, lat));

    // 星盘轮图：上升点固定在左侧（9 点钟方向），黄道逆时针展开
    const asc = c.asc.lon;
    const P = (lonDeg, r) => {
      const a = (180 - (lonDeg - asc)) * Math.PI / 180;
      return [160 + r * Math.cos(a), 160 - r * Math.sin(a)];
    };
    let svg = '<svg viewBox="0 0 320 320" class="wheel" role="img" aria-label="星盘轮图">';
    svg += '<circle cx="160" cy="160" r="150" class="wring"/><circle cx="160" cy="160" r="112" class="wring"/>';
    for (let s = 0; s < 12; s++) {
      const [x1, y1] = P(s * 30, 112);
      const [x2, y2] = P(s * 30, 150);
      svg += `<line x1="${x1.toFixed(1)}" y1="${y1.toFixed(1)}" x2="${x2.toFixed(1)}" y2="${y2.toFixed(1)}" class="wline"/>`;
      const [gx, gy] = P(s * 30 + 15, 131);
      svg += `<text x="${gx.toFixed(1)}" y="${(gy + 7).toFixed(1)}" class="wglyph">${['♈','♉','♊','♋','♌','♍','♎','♏','♐','♑','♒','♓'][s]}</text>`;
      const [hx, hy] = P(asc + s * 30, 96);
      const [hx2, hy2] = P(asc + s * 30, 112);
      svg += `<line x1="${hx.toFixed(1)}" y1="${hy.toFixed(1)}" x2="${hx2.toFixed(1)}" y2="${hy2.toFixed(1)}" class="whouse"/>`;
    }
    for (const p of c.planets) {
      const [x, y] = P(p.lon, 88);
      svg += `<text x="${x.toFixed(1)}" y="${(y + 6).toFixed(1)}" class="wplanet">${p.glyph}</text>`;
    }
    const [ax, ay] = P(asc, 156);
    svg += `<text x="${ax.toFixed(1)}" y="${(ay + 4).toFixed(1)}" class="wasc">ASC</text>`;
    const [mx, my] = P(c.mc.lon, 156);
    svg += `<text x="${mx.toFixed(1)}" y="${(my + 4).toFixed(1)}" class="wasc">MC</text>`;
    svg += '</svg>';

    const rows = c.planets.map(p => `
      <div class="ntrow"><span class="ntglyph">${p.glyph}</span>
      <span class="ntname">${p.name}${p.retro ? ' <em class="ntretro">逆行</em>' : ''}</span>
      <span>${p.sign_glyph} ${p.sign} ${p.sign_deg.toFixed(1)}°</span>
      <span class="nthouse">第 ${p.house} 宫</span>
      ${p.cusp ? '<span class="ntcusp">临界</span>' : ''}</div>`).join('');
    const asp = c.aspects.length
      ? c.aspects.map(a => `<span class="ntasp">${a.a} ${a.kind} ${a.b}（${a.orb.toFixed(1)}°）</span>`).join('')
      : '<span class="fineprint">主要相位暂无（容许度内）</span>';
    el('ntOut').innerHTML = `
      ${svg}
      <p class="ntbig serif">☉ ${c.planets[0].sign} · ☽ ${c.planets[1].sign} · 上升 ${c.asc.sign}</p>
      <p>${c.sun_text}</p><p>${c.moon_text}</p><p>${c.asc_text}</p>
      <div class="ntrows">${rows}</div>
      <p class="ntasps">${asp}</p>
      <p class="fineprint">中天 ${c.mc.sign_glyph} ${c.mc.sign} ${c.mc.sign_deg.toFixed(1)}°；等宫制（自上升每 30° 一宫）。
      标「临界」的行星距星座边界不足 0.3°，落座仅供参考。时间为东八区（北京时间）；
      若出生地时区不同，请先把出生时间换算成北京时间再排。</p>`;
    el('ntOut').classList.remove('hidden');
  };
}
