/* @ts-self-types="./xuanji_web.d.ts" */

/**
 * 今日黄历：入参为公历年月日，返回 JSON 字符串
 * @param {number} y
 * @param {number} m
 * @param {number} d
 * @returns {string}
 */
export function almanac_json(y, m, d) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.almanac_json(y, m, d);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * 八字排盘：出生年月日时分 + 性别（1 男 0 女），返回 JSON 字符串
 * @param {number} y
 * @param {number} m
 * @param {number} d
 * @param {number} hour
 * @param {number} minute
 * @param {number} gender
 * @returns {string}
 */
export function bazi_json(y, m, d, hour, minute, gender) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.bazi_json(y, m, d, hour, minute, gender);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * 称骨：出生年月日时分，返回 JSON
 * @param {number} y
 * @param {number} m
 * @param {number} d
 * @param {number} hour
 * @param {number} minute
 * @returns {string}
 */
export function bone_json(y, m, d, hour, minute) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.bone_json(y, m, d, hour, minute);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * 答案之书：entropy 为浏览器传入的随机数
 * @param {number} entropy
 * @returns {string}
 */
export function book_answer(entropy) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.book_answer(entropy);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * 今日一签：同一天抽到的签固定，返回 JSON 字符串
 * @param {number} y
 * @param {number} m
 * @param {number} d
 * @returns {string}
 */
export function daily_qian_json(y, m, d) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.daily_qian_json(y, m, d);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * 每日时辰吉凶：入参为公历年月日，返回十二时辰 JSON 数组
 * @param {number} y
 * @param {number} m
 * @param {number} d
 * @returns {string}
 */
export function day_hours_json(y, m, d) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.day_hours_json(y, m, d);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * 万事择日：起止公历日期 + 事项 + 两个避冲生肖（空串不限），返回 JSON 数组
 * @param {number} y1
 * @param {number} m1
 * @param {number} d1
 * @param {number} y2
 * @param {number} m2
 * @param {number} d2
 * @param {string} activity
 * @param {string} avoid1
 * @param {string} avoid2
 * @returns {string}
 */
export function day_search_json(y1, m1, d1, y2, m2, d2, activity, avoid1, avoid2) {
    let deferred10_0;
    let deferred10_1;
    try {
        const val4 = +y1;
        const val5 = +m1;
        const val6 = +d1;
        const val7 = +y2;
        const val8 = +m2;
        const val9 = +d2;
        let ptr0, len0, ptr1, len1, ptr2, len2;
        try {
            ptr0 = passStringToWasm0(activity, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            len0 = WASM_VECTOR_LEN;
            ptr1 = passStringToWasm0(avoid1, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            len1 = WASM_VECTOR_LEN;
            ptr2 = passStringToWasm0(avoid2, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            len2 = WASM_VECTOR_LEN;
        } catch (e) {
            if (ptr2) { wasm.__wbindgen_free(ptr2, len2 * 1, 1); }
            if (ptr1) { wasm.__wbindgen_free(ptr1, len1 * 1, 1); }
            if (ptr0) { wasm.__wbindgen_free(ptr0, len0 * 1, 1); }
            throw e;
        }
        const ret = wasm.day_search_json(val4, val5, val6, val7, val8, val9, ptr0, len0, ptr1, len1, ptr2, len2);
        deferred10_0 = ret[0];
        deferred10_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred10_0, deferred10_1, 1);
    }
}

/**
 * 解梦词条库（原创），返回 JSON
 * @returns {string}
 */
export function dreams_json() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.dreams_json();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * 流年九宫飞星：干支年（立春为界），返回 JSON
 * @param {number} year
 * @returns {string}
 */
export function feixing_json(year) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.feixing_json(year);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * 合婚：两人出生年月日时分，返回 JSON
 * @param {number} ay
 * @param {number} am
 * @param {number} ad
 * @param {number} ah
 * @param {number} amin
 * @param {number} by
 * @param {number} bm
 * @param {number} bd
 * @param {number} bh
 * @param {number} bmin
 * @returns {string}
 */
export function hehun_json(ay, am, ad, ah, amin, by, bm, bd, bh, bmin) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.hehun_json(ay, am, ad, ah, amin, by, bm, bd, bh, bmin);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * 六爻起卦装卦：种子 + 起卦日期（公历），返回 JSON
 * @param {number} seed
 * @param {number} y
 * @param {number} m
 * @param {number} d
 * @returns {string}
 */
export function liuyao_json(seed, y, m, d) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.liuyao_json(seed, y, m, d);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * 梅花时间起卦：公历年月日 + 小时，返回 JSON
 * @param {number} y
 * @param {number} m
 * @param {number} d
 * @param {number} hour
 * @returns {string}
 */
export function meihua_json(y, m, d, hour) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.meihua_json(y, m, d, hour);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * 奇门排盘：公历日期 + 小时，返回 JSON
 * @param {number} y
 * @param {number} m
 * @param {number} d
 * @param {number} hour
 * @returns {string}
 */
export function qimen_json(y, m, d, hour) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.qimen_json(y, m, d, hour);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * 犯太岁：公历年份 + 本人生肖（空串只看全年名单），返回 JSON
 * @param {number} year
 * @param {string} zodiac
 * @returns {string}
 */
export function taisui_json(year, zodiac) {
    let deferred3_0;
    let deferred3_1;
    try {
        const val2 = +year;
        const ptr0 = passStringToWasm0(zodiac, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.taisui_json(val2, ptr0, len0);
        deferred3_0 = ret[0];
        deferred3_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred3_0, deferred3_1, 1);
    }
}

/**
 * 婚嫁择日：起止公历日期 + 新郎/新娘生肖（空串不限），返回 JSON 数组
 * @param {number} y1
 * @param {number} m1
 * @param {number} d1
 * @param {number} y2
 * @param {number} m2
 * @param {number} d2
 * @param {string} groom
 * @param {string} bride
 * @returns {string}
 */
export function wedding_json(y1, m1, d1, y2, m2, d2, groom, bride) {
    let deferred9_0;
    let deferred9_1;
    try {
        const val3 = +y1;
        const val4 = +m1;
        const val5 = +d1;
        const val6 = +y2;
        const val7 = +m2;
        const val8 = +d2;
        let ptr0, len0, ptr1, len1;
        try {
            ptr0 = passStringToWasm0(groom, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            len0 = WASM_VECTOR_LEN;
            ptr1 = passStringToWasm0(bride, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            len1 = WASM_VECTOR_LEN;
        } catch (e) {
            if (ptr1) { wasm.__wbindgen_free(ptr1, len1 * 1, 1); }
            if (ptr0) { wasm.__wbindgen_free(ptr0, len0 * 1, 1); }
            throw e;
        }
        const ret = wasm.wedding_json(val3, val4, val5, val6, val7, val8, ptr0, len0, ptr1, len1);
        deferred9_0 = ret[0];
        deferred9_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred9_0, deferred9_1, 1);
    }
}

/**
 * 周易抽卦：种子（页面每次点击传入新随机数），返回 JSON
 * @param {number} seed
 * @returns {string}
 */
export function zhouyi_draw_json(seed) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.zhouyi_draw_json(seed);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * 紫微排盘：出生年月日时分 + 性别（1 男 0 女），返回 JSON
 * @param {number} y
 * @param {number} m
 * @param {number} d
 * @param {number} hour
 * @param {number} minute
 * @param {number} gender
 * @returns {string}
 */
export function ziwei_json(y, m, d, hour, minute, gender) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.ziwei_json(y, m, d, hour, minute, gender);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * 紫微流年：出生信息 + 流年年份，返回 JSON
 * @param {number} y
 * @param {number} m
 * @param {number} d
 * @param {number} hour
 * @param {number} minute
 * @param {number} gender
 * @param {number} year
 * @returns {string}
 */
export function ziwei_liunian_json(y, m, d, hour, minute, gender, year) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.ziwei_liunian_json(y, m, d, hour, minute, gender, year);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_4c88d8bf264fd984: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            table.set(0, undefined);
            table.set(1024, undefined);
            table.set(1025, null);
            table.set(1026, true);
            table.set(1027, false);
        },
    };
    return {
        __proto__: null,
        "./xuanji_web_bg.js": import0,
    };
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(bytes) {
    numBytesDecoded += bytes.length;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true });
        cachedTextDecoder.decode();
        numBytesDecoded = bytes.length;
    }
    return cachedTextDecoder.decode(bytes);
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (!module.ok) {
            throw new Error(`failed to fetch Wasm: ${module.status} ${module.statusText} fetching '${module.url}'`);
        }

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

let __wbg_init_promise;
async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;
    if (__wbg_init_promise === undefined) {
        __wbg_init_promise = __wbg_init_once(module_or_path);
        __wbg_init_promise.catch(() => { __wbg_init_promise = undefined; });
    }
    return __wbg_init_promise;
}

async function __wbg_init_once(module_or_path) {

    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('xuanji_web_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
