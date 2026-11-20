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
 * 答案之书：entropy 为浏览器传入的随机数
 */
export function book_answer(entropy: number): string;

/**
 * 今日一签：同一天抽到的签固定，返回 JSON 字符串
 */
export function daily_qian_json(y: number, m: number, d: number): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly almanac_json: (a: number, b: number, c: number) => [number, number];
    readonly bazi_json: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
    readonly book_answer: (a: number) => [number, number];
    readonly daily_qian_json: (a: number, b: number, c: number) => [number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
