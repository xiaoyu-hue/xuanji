/* tslint:disable */
/* eslint-disable */

/**
 * 今日黄历：入参为公历年月日，返回 JSON 字符串
 */
export function almanac_json(y: number, m: number, d: number): string;

/**
 * 八字排盘：出生年月日时分 + 性别（1 男 0 女），返回 JSON 字符串
 */
export function bazi_json(y: number, m: number, d: number, hour: number, minute: number, gender: number): string;

/**
 * 称骨：出生年月日时分，返回 JSON
 */
export function bone_json(y: number, m: number, d: number, hour: number, minute: number): string;

/**
 * 答案之书：entropy 为浏览器传入的随机数
 */
export function book_answer(entropy: number): string;

/**
 * 今日一签：同一天抽到的签固定，返回 JSON 字符串
 */
export function daily_qian_json(y: number, m: number, d: number): string;

/**
 * 每日时辰吉凶：入参为公历年月日，返回十二时辰 JSON 数组
 */
export function day_hours_json(y: number, m: number, d: number): string;

/**
 * 万事择日：起止公历日期 + 事项 + 两个避冲生肖（空串不限），返回 JSON 数组
 */
export function day_search_json(y1: number, m1: number, d1: number, y2: number, m2: number, d2: number, activity: string, avoid1: string, avoid2: string): string;

/**
 * 解梦词条库（原创），返回 JSON
 */
export function dreams_json(): string;

/**
 * 流年九宫飞星：干支年（立春为界），返回 JSON
 */
export function feixing_json(year: number): string;

/**
 * 合婚：两人出生年月日时分，返回 JSON
 */
export function hehun_json(ay: number, am: number, ad: number, ah: number, amin: number, by: number, bm: number, bd: number, bh: number, bmin: number): string;

/**
 * 六爻起卦装卦：种子 + 起卦日期（公历），返回 JSON
 */
export function liuyao_json(seed: number, y: number, m: number, d: number): string;

/**
 * 梅花时间起卦：公历年月日 + 小时，返回 JSON
 */
export function meihua_json(y: number, m: number, d: number, hour: number): string;

/**
 * 奇门排盘：公历日期 + 小时，返回 JSON
 */
export function qimen_json(y: number, m: number, d: number, hour: number): string;

/**
 * 犯太岁：公历年份 + 本人生肖（空串只看全年名单），返回 JSON
 */
export function taisui_json(year: number, zodiac: string): string;

/**
 * 婚嫁择日：起止公历日期 + 新郎/新娘生肖（空串不限），返回 JSON 数组
 */
export function wedding_json(y1: number, m1: number, d1: number, y2: number, m2: number, d2: number, groom: string, bride: string): string;

/**
 * 周易抽卦：种子（页面每次点击传入新随机数），返回 JSON
 */
export function zhouyi_draw_json(seed: number): string;

/**
 * 紫微排盘：出生年月日时分 + 性别（1 男 0 女），返回 JSON
 */
export function ziwei_json(y: number, m: number, d: number, hour: number, minute: number, gender: number): string;

/**
 * 紫微流年：出生信息 + 流年年份，返回 JSON
 */
export function ziwei_liunian_json(y: number, m: number, d: number, hour: number, minute: number, gender: number, year: number): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly almanac_json: (a: number, b: number, c: number) => [number, number];
    readonly bazi_json: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
    readonly bone_json: (a: number, b: number, c: number, d: number, e: number) => [number, number];
    readonly book_answer: (a: number) => [number, number];
    readonly daily_qian_json: (a: number, b: number, c: number) => [number, number];
    readonly day_hours_json: (a: number, b: number, c: number) => [number, number];
    readonly day_search_json: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number) => [number, number];
    readonly dreams_json: () => [number, number];
    readonly feixing_json: (a: number) => [number, number];
    readonly hehun_json: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => [number, number];
    readonly liuyao_json: (a: number, b: number, c: number, d: number) => [number, number];
    readonly meihua_json: (a: number, b: number, c: number, d: number) => [number, number];
    readonly qimen_json: (a: number, b: number, c: number, d: number) => [number, number];
    readonly taisui_json: (a: number, b: number, c: number) => [number, number];
    readonly wedding_json: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => [number, number];
    readonly zhouyi_draw_json: (a: number) => [number, number];
    readonly ziwei_json: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
    readonly ziwei_liunian_json: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
