// 六爻：金钱卦起卦 + 装卦（卦宫/世应/纳甲/六亲/六神/旬空）
import { el, ctx } from './ctx.js?v=6.0.1';

function lineBar(l) {
  return l.yang
    ? '<span class="gline yang"></span>'
    : '<span class="gline yin"><i></i><i></i></span>';
}

export function initLiuyao(w) {
  el('btnLiuyao').addEventListener('click', () => {
    const seed = (Date.now() % 2147483647) ^ Math.floor(Math.random() * 1e9);
    const r = JSON.parse(w.liuyao_json(seed, ctx.Y, ctx.M, ctx.D));
    const rows = [...r.lines].reverse().map(l => `
      <div class="lyrow">
        <span class="lyshen">${l.shen}</span>
        <span class="lybar">${lineBar(l)}${l.changing ? '<b class="lychg">○</b>' : ''}</span>
        <span class="lyqin">${l.liuqin} ${l.zhi}${l.wuxing}</span>
        <span class="lymark">${l.is_shi ? '世' : ''}${l.is_ying ? '应' : ''}${l.kong ? ' 空' : ''}</span>
      </div>`).join('');
    let bian = '';
    if (r.bian) {
      const rev = [...r.lines].reverse();
      const bl = [...r.bian_lines].reverse().map((b, i) => `
        <div class="lyrow lybrow">
          <span class="lyshen"></span>
          <span class="lybar">${lineBar({ yang: rev[i].yang !== b.changing })}</span>
          <span class="lyqin">${b.liuqin} ${b.zhi}${b.wuxing}</span>
          <span class="lymark">${b.changing ? '变' : ''}</span>
        </div>`).join('');
      bian = `<div class="bianbox"><div class="biantitle">之卦 ${r.bian.full}（第 ${r.bian.n} 卦）</div>${bl}</div>`;
    }
    el('lyOut').innerHTML = `
      <div class="guatitle">${r.ben.full}（第 ${r.ben.n} 卦）· ${r.palace}宫（${r.palace_wuxing}）</div>
      <div class="fineprint">起卦日 ${r.day_ganzhi} · 月建${r.yue_jian} · 世爻在${['初', '二', '三', '四', '五', '上'][r.shi - 1]}、应爻在${['初', '二', '三', '四', '五', '上'][r.ying - 1]} · 旬空${r.kong_wang.join('')}</div>
      <div class="lytable">${rows}</div>
      ${bian}
      <p class="dayi">${r.ben.dayi}</p>
      <p class="fineprint">本卡为「装卦」：定宫、世应、纳甲、六亲、六神、旬空；断卦还需结合用神与月建日辰旺衰，不在此列。</p>`;
  });
}
