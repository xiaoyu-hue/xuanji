//! 西洋星盘（西方馆压轴）：本命盘——十大行星黄道落座、上升/中天、
//! 等宫制十二宫、主要相位。星历用 JPL 近似开普勒轨道根数（1800–2050，
//! 精度角分级）＋月亮低精度理论（Schlyter 摄动主项）；与瑞士星历
//! （pyswisseph）逐行星对拍验证（见开发文档）。落座临界（距星座边界
//! 0.3° 内）页面会提示仅供参考。解读为本站原创。

use serde::Serialize;

const SIGNS: [&str; 12] = [
    "白羊座", "金牛座", "双子座", "巨蟹座", "狮子座", "处女座", "天秤座", "天蝎座",
    "射手座", "摩羯座", "水瓶座", "双鱼座",
];
const SIGN_GLYPHS: [&str; 12] = ["♈", "♉", "♊", "♋", "♌", "♍", "♎", "♏", "♐", "♑", "♒", "♓"];

fn norm360(x: f64) -> f64 {
    let r = x % 360.0;
    if r < 0.0 {
        r + 360.0
    } else {
        r
    }
}
fn rad(d: f64) -> f64 {
    d.to_radians()
}
fn deg(r: f64) -> f64 {
    r.to_degrees()
}

/// 儒略日（UT）
fn julian_day(y: i64, m: i64, d: i64, ut_hours: f64) -> f64 {
    let (yy, mm) = if m <= 2 { (y - 1, m + 12) } else { (y, m) };
    let a = yy / 100;
    let b = 2 - a + a / 4;
    let jd_day = (365.25 * (yy + 4716) as f64).floor()
        + (30.6001 * (mm + 1) as f64).floor()
        + d as f64
        + b as f64
        - 1524.5;
    jd_day + ut_hours / 24.0
}

/// JPL 近似根数：(a[AU], e, I[°], L[°], ϖ[°], Ω[°]) 及每儒略世纪变化率
struct Elem {
    a: f64,
    e: f64,
    i: f64,
    l: f64,
    peri: f64,
    node: f64,
}
fn elem_at(base: [f64; 6], rates: [f64; 6], t: f64) -> Elem {
    Elem {
        a: base[0] + rates[0] * t,
        e: base[1] + rates[1] * t,
        i: base[2] + rates[2] * t,
        l: base[3] + rates[3] * t,
        peri: base[4] + rates[4] * t,
        node: base[5] + rates[5] * t,
    }
}

/// 行星日心黄道直角坐标（AU）
fn helio_xyz(el: &Elem) -> (f64, f64, f64) {
    let w = el.peri - el.node; // 近点幅角
    let m_anom = norm360(el.l - el.peri);
    let mr = rad(m_anom);
    let mut ea = mr + el.e * mr.sin() * (1.0 + el.e * mr.cos());
    for _ in 0..12 {
        ea -= (ea - el.e * ea.sin() - mr) / (1.0 - el.e * ea.cos());
    }
    let xv = el.a * (ea.cos() - el.e);
    let yv = el.a * (1.0 - el.e * el.e).sqrt() * ea.sin();
    let v = yv.atan2(xv);
    let r = (xv * xv + yv * yv).sqrt();
    let (cn, sn) = (rad(el.node).cos(), rad(el.node).sin());
    let (ci, si) = (rad(el.i).cos(), rad(el.i).sin());
    let (cvw, svw) = ((v + rad(w)).cos(), (v + rad(w)).sin());
    (
        r * (cn * cvw - sn * svw * ci),
        r * (sn * cvw + cn * svw * ci),
        r * (svw * si),
    )
}

const PLANETS: [(&str, &str, [f64; 6], [f64; 6]); 8] = [
    ("水星", "☿",
     [0.38709927, 0.20563593, 7.00497902, 252.25032350, 77.45779628, 48.33076593],
     [0.00000037, 0.00001906, -0.00594749, 149472.67411175, 0.16047689, -0.12534081]),
    ("金星", "♀",
     [0.72333566, 0.00677672, 3.39467605, 181.97909950, 131.60246718, 76.67984255],
     [0.00000390, -0.00004107, -0.00078890, 58517.81538729, 0.00268329, -0.27769418]),
    ("火星", "♂",
     [1.52371034, 0.09339410, 1.84969142, -4.55343205, -23.94362959, 49.55953891],
     [0.00001847, 0.00007882, -0.00813131, 19140.30268499, 0.44441088, -0.29257343]),
    ("木星", "♃",
     [5.20288700, 0.04838624, 1.30439695, 34.39644051, 14.72847983, 100.47390909],
     [-0.00011607, -0.00013253, -0.00183714, 3034.74612775, 0.21252668, 0.20469106]),
    ("土星", "♄",
     [9.53667594, 0.05386179, 2.48599187, 49.95424423, 92.59887831, 113.66242448],
     [-0.00125060, -0.00050991, 0.00193609, 1222.49362201, -0.41897216, -0.28867794]),
    ("天王星", "⛢",
     [19.18916464, 0.04725744, 0.77263783, 313.23810451, 170.95427630, 74.01692503],
     [-0.00196176, -0.00004397, -0.00242939, 428.48202785, 0.40805281, 0.04240589]),
    ("海王星", "♆",
     [30.06992276, 0.00859048, 1.77004347, -55.12002969, 44.96476227, 131.78422574],
     [0.00026291, 0.00005105, 0.00035372, 218.45945325, -0.32241464, -0.00508664]),
    ("冥王星", "♇",
     [39.48211675, 0.24882730, 17.14001206, 238.92903833, 224.06891629, 110.30393684],
     [-0.00031596, 0.00005170, 0.00004818, 145.20780515, -0.04062942, -0.01183482]),
];
const EARTH: ([f64; 6], [f64; 6]) = (
    [1.00000261, 0.01671123, -0.00001531, 100.46457166, 102.93768193, 0.0],
    [0.00000562, -0.00004392, -0.01294668, 35999.37244981, 0.32327364, 0.0],
);

/// 月亮地心黄经（Schlyter 根数＋摄动主项），d 为自 2000 Jan 0.0 起的天数
fn moon_lon(jd: f64) -> f64 {
    let d = jd - 2451543.5;
    let n = 125.1228 - 0.0529538083 * d;
    let w = 318.0634 + 0.1643573223 * d;
    let e = 0.054900f64;
    let m_anom = norm360(115.3654 + 13.0649929509 * d);
    let mr = rad(m_anom);
    let mut ea = mr + e * mr.sin() * (1.0 + e * mr.cos());
    for _ in 0..12 {
        ea -= (ea - e * ea.sin() - mr) / (1.0 - e * ea.cos());
    }
    let xv = 60.2666 * (ea.cos() - e);
    let yv = 60.2666 * (1.0 - e * e).sqrt() * ea.sin();
    let v = yv.atan2(xv);
    let (cn, sn) = (rad(n).cos(), rad(n).sin());
    let (ci, si) = (rad(5.1454).cos(), rad(5.1454).sin());
    let (cvw, svw) = ((v + rad(w)).cos(), (v + rad(w)).sin());
    let x = xv.hypot(yv) * (cn * cvw - sn * svw * ci);
    let y = xv.hypot(yv) * (sn * cvw + cn * svw * ci);
    let mut lon = norm360(deg(y.atan2(x)));
    // 摄动主项
    let ms = norm360(356.0470 + 0.9856002585 * d);
    let mm = m_anom;
    let lm = norm360(n + w + mm);
    let ls = norm360(280.4665 + 0.98564736 * d);
    let elong = norm360(lm - ls);
    let f = norm360(lm - n);
    let s = |x: f64| rad(x).sin();
    lon += -1.274 * s(mm - 2.0 * elong)
        + 0.658 * s(2.0 * elong)
        - 0.186 * s(ms)
        - 0.059 * s(2.0 * mm - 2.0 * elong)
        - 0.057 * s(mm - 2.0 * elong + ms)
        + 0.053 * s(mm + 2.0 * elong)
        + 0.046 * s(2.0 * elong - ms)
        + 0.041 * s(mm - ms)
        - 0.035 * s(elong)
        - 0.031 * s(mm + ms)
        - 0.015 * s(2.0 * f - 2.0 * elong)
        + 0.011 * s(mm - 4.0 * elong);
    norm360(lon)
}

/// 某时刻十大行星地心黄经（太阳…冥王星）＋是否逆行（数值微分）
fn planet_lons(jd: f64) -> Vec<(String, String, f64)> {
    let t = (jd - 2451545.0) / 36525.0;
    // JPL 根数为 J2000 平黄道，须加岁差转到当日平黄道（月亮理论本身即当日
    // 坐标，不加）；未加时误差随年份距 2000 的距离线性增长（约 50.3″/年）
    let prec = (5029.0966 * t + 1.11113 * t * t) / 3600.0;
    let earth = helio_xyz(&elem_at(EARTH.0, EARTH.1, t));
    let sun_lon = norm360(deg(earth.1.atan2(earth.0)) + 180.0 + prec);
    let mut out = vec![
        ("太阳".to_string(), "☉".to_string(), sun_lon),
        ("月亮".to_string(), "☽".to_string(), moon_lon(jd)),
    ];
    for (name, glyph, base, rates) in PLANETS.iter() {
        let p = helio_xyz(&elem_at(*base, *rates, t));
        let lon = norm360(deg((p.1 - earth.1).atan2(p.0 - earth.0)) + prec);
        out.push((name.to_string(), glyph.to_string(), lon));
    }
    out
}

#[derive(Serialize)]
pub struct PlanetPos {
    pub name: String,
    pub glyph: String,
    pub lon: f64,
    pub sign: String,
    pub sign_glyph: String,
    pub sign_deg: f64,
    pub house: u32,
    pub retro: bool,
    /// 距星座边界 0.3° 内（落座临界，提示仅供参考）
    pub cusp: bool,
}

#[derive(Serialize)]
pub struct ChartPoint {
    pub lon: f64,
    pub sign: String,
    pub sign_glyph: String,
    pub sign_deg: f64,
}

#[derive(Serialize)]
pub struct Aspect {
    pub a: String,
    pub b: String,
    pub kind: String,
    pub orb: f64,
}

#[derive(Serialize)]
pub struct NatalChart {
    pub planets: Vec<PlanetPos>,
    pub asc: ChartPoint,
    pub mc: ChartPoint,
    pub houses: Vec<f64>,
    pub aspects: Vec<Aspect>,
    pub sun_text: String,
    pub moon_text: String,
    pub asc_text: String,
}

fn point(lon: f64) -> ChartPoint {
    let idx = ((lon / 30.0).floor() as usize) % 12;
    ChartPoint {
        lon: (lon * 100.0).round() / 100.0,
        sign: SIGNS[idx].to_string(),
        sign_glyph: SIGN_GLYPHS[idx].to_string(),
        sign_deg: ((lon % 30.0) * 10.0).round() / 10.0,
    }
}

const SUN_TEXT: [&str; 12] = [
    "太阳白羊：生命力直来直去，先行动再调整，骨子里要赢也要痛快。",
    "太阳金牛：稳扎稳打的建设者，享受感官与成果，认定的事九头牛拉不回。",
    "太阳双子：好奇心永动机，靠信息和交流充电，一成不变最难熬。",
    "太阳巨蟹：外柔内韧，守护家人与地盘，直觉准得像天气预报。",
    "太阳狮子：天生的主角光环，需要舞台与掌声，慷慨也爱面子。",
    "太阳处女：细节里的完美主义，靠靠谱和手艺赢得尊重，操心命。",
    "太阳天秤：审美与分寸感一流，讨厌冲突，关系里求一个平衡。",
    "太阳天蝎：洞察人心的高手，爱憎分明，不出手则已，出手必到位。",
    "太阳射手：心在远方，信奉体验与真理，讨厌被束缚和说教别人时最起劲。",
    "太阳摩羯：目标感极强的长跑者，靠时间和自律把野心熬成地位。",
    "太阳水瓶：天生的局外人视角，想法超前，合群但绝不随大流。",
    "太阳双鱼：共情与想象力泛滥，艺术感强，需要独处给心灵排水。",
];
const MOON_TEXT: [&str; 12] = [
    "月亮白羊：情绪来得快去得快，需要即时回应，受了委屈睡一觉就好。",
    "月亮金牛：安全感来自稳定与美食，情绪节奏慢，认定的人最长情。",
    "月亮双子：心里住着话痨，情绪靠说出来消化，新鲜感是安神药。",
    "月亮巨蟹：月亮回庙，感受力最强，家的氛围直接决定心情晴雨。",
    "月亮狮子：需要被看见被夸，情绪表达戏剧化，哄一哄就雨过天晴。",
    "月亮处女：焦虑藏在细节里，把环境收拾整齐，心也就顺了。",
    "月亮天秤：怕孤单怕难看，情绪需要伴侣与美感来安抚，纠结是日常。",
    "月亮天蝎：感受深不见底，信任给得很慢，占有欲与直觉一样强。",
    "月亮射手：情绪需要空间和远方，闷在原地最容易低落，出门就满血。",
    "月亮摩羯：情绪不轻易外露，靠做成事情获得安全感，其实很需要肯定。",
    "月亮水瓶：情绪抽离派，难过也要讲道理，朋友比家人更懂自己的心。",
    "月亮双鱼：共情力满格，分不清是自己的还是别人的情绪，需要边界。",
];
const ASC_TEXT: [&str; 12] = [
    "上升白羊：第一印象干练直接，走路带风，开场自带气势。",
    "上升金牛：给人稳重可靠的初印象，不紧不慢，自带质感。",
    "上升双子：显得机灵健谈，少年感强，跟谁都能聊两句。",
    "上升巨蟹：气质温和亲切，看起来好相处，自带保护色。",
    "上升狮子：出场自带聚光灯，体面大方，让人一眼记住。",
    "上升处女：显得整洁利落、做事有条理，可靠感拉满。",
    "上升天秤：颜值与礼貌双在线，第一印象优雅好相处。",
    "上升天蝎：神秘高冷，眼神有压迫感，让人不敢轻易靠近。",
    "上升射手：给人爽朗随和的印象，笑声先到人后到。",
    "上升摩羯：显得成熟稳重，比实际年龄更像个当家人。",
    "上升水瓶：气质疏离独特，不按套路出牌，特立独行。",
    "上升双鱼：眼神温柔梦幻，看起来好说话，自带艺术气息。",
];

/// 排本命盘：公历日期 + 北京时间（时，分）+ 出生地经纬度（东经/北纬为正）。
/// 时区按东八区处理（中国大陆/台北等地）；其他时区请先把时间换算为东八区。
pub fn natal_chart(y: i64, m: i64, d: i64, hh: i64, mm: i64, lon_e: f64, lat: f64) -> NatalChart {
    let ut = hh as f64 + mm as f64 / 60.0 - 8.0;
    let jd = julian_day(y, m, d, ut);
    let t = (jd - 2451545.0) / 36525.0;
    let eps = rad(23.4392911 - 0.0127778 * t);
    // 恒星时
    let gmst = norm360(280.46061837 + 360.98564736629 * (jd - 2451545.0));
    let lst = norm360(gmst + lon_e);
    let ramc = rad(lst);
    // MC：tan(MC) = tan(RAMC)/cosε，atan2 形式保证与 RAMC 同象限
    let mc_lon = norm360(deg((ramc.sin() / eps.cos()).atan2(ramc.cos())));
    let asc_lon = {
        // 此式给出下降点对面的上升点：atan2 结果即上升黄经（与金标准对拍确认，
        // 早先误加 180° 得的是下降点）
        let y = ramc.cos();
        let x = -(ramc.sin() * eps.cos() + rad(lat).tan() * eps.sin());
        norm360(deg(y.atan2(x)))
    };

    let now = planet_lons(jd);
    let before = planet_lons(jd - 0.5);
    let after = planet_lons(jd + 0.5);
    let mut planets = Vec::new();
    for (i, (name, glyph, lon)) in now.iter().enumerate() {
        let mut dl = after[i].2 - before[i].2;
        if dl > 180.0 {
            dl -= 360.0;
        } else if dl < -180.0 {
            dl += 360.0;
        }
        let retro = i >= 2 && dl < 0.0;
        let idx = ((lon / 30.0).floor() as usize) % 12;
        let in_sign = lon % 30.0;
        let house = ((norm360(lon - asc_lon) / 30.0).floor() as u32) + 1;
        planets.push(PlanetPos {
            name: name.clone(),
            glyph: glyph.clone(),
            lon: (lon * 100.0).round() / 100.0,
            sign: SIGNS[idx].to_string(),
            sign_glyph: SIGN_GLYPHS[idx].to_string(),
            sign_deg: (in_sign * 10.0).round() / 10.0,
            house,
            retro,
            cusp: in_sign < 0.3 || in_sign > 29.7,
        });
    }
    let houses: Vec<f64> = (0..12)
        .map(|k| (norm360(asc_lon + 30.0 * k as f64) * 100.0).round() / 100.0)
        .collect();
    // 主要相位
    let kinds = [
        (0.0, "合相"),
        (60.0, "六合"),
        (90.0, "刑"),
        (120.0, "拱"),
        (180.0, "冲"),
    ];
    let mut aspects = Vec::new();
    for i in 0..now.len() {
        for j in (i + 1)..now.len() {
            let mut diff = (now[i].2 - now[j].2).abs();
            if diff > 180.0 {
                diff = 360.0 - diff;
            }
            for (ang, kind) in kinds.iter() {
                let orb = (diff - ang).abs();
                let limit = if i < 2 || j < 2 { 8.0 } else { 5.0 };
                if orb <= limit {
                    aspects.push(Aspect {
                        a: now[i].0.clone(),
                        b: now[j].0.clone(),
                        kind: kind.to_string(),
                        orb: (orb * 10.0).round() / 10.0,
                    });
                }
            }
        }
    }
    aspects.sort_by(|a, b| a.orb.partial_cmp(&b.orb).unwrap());
    let sun_idx = ((now[0].2 / 30.0).floor() as usize) % 12;
    let moon_idx = ((now[1].2 / 30.0).floor() as usize) % 12;
    let asc_idx = ((asc_lon / 30.0).floor() as usize) % 12;
    NatalChart {
        planets,
        asc: point(asc_lon),
        mc: point(mc_lon),
        houses,
        aspects,
        sun_text: SUN_TEXT[sun_idx].to_string(),
        moon_text: MOON_TEXT[moon_idx].to_string(),
        asc_text: ASC_TEXT[asc_idx].to_string(),
    }
}
