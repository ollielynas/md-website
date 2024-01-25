/* tslint:disable */
/* eslint-disable */
/**
*/
export function console_error_panic_hook_set(): void;
/**
* @param {string} path
* @returns {Promise<void>}
*/
export function collapse(path: string): Promise<void>;
/**
* todo: make this function return a result
* @returns {Promise<void>}
*/
export function update_nav(): Promise<void>;
/**
* @param {string} file
* @returns {Promise<string>}
*/
export function load_gzip(file: string): Promise<string>;
/**
* @param {string} file
* @returns {Promise<void>}
*/
export function load_md(file: string): Promise<void>;
/**
* @returns {Promise<void>}
*/
export function load_style(): Promise<void>;
/**
* @returns {Promise<void>}
*/
export function update_from_hash(): Promise<void>;
/**
* @returns {Promise<void>}
*/
export function rs_onload(): Promise<void>;
/**
* @returns {Promise<void>}
*/
export function clicked_scroll(): Promise<void>;
/**
* @returns {Promise<void>}
*/
export function show_nav(): Promise<void>;
/**
* @returns {Promise<void>}
*/
export function show_content(): Promise<void>;
/**
* @param {string} input
* @returns {Promise<void>}
*/
export function search_results(input: string): Promise<void>;
/**
* @param {string} input
* @returns {Promise<void>}
*/
export function search_results_big(input: string): Promise<void>;
/**
* @returns {Promise<void>}
*/
export function back_arrow(): Promise<void>;
/**
* @returns {Promise<void>}
*/
export function forward_arrow(): Promise<void>;
/**
* @returns {Promise<void>}
*/
export function on_finish_animation(): Promise<void>;
/**
* @returns {Promise<void>}
*/
export function startle_bird(): Promise<void>;
/**
* @returns {Promise<void>}
*/
export function update_bird_target_location(): Promise<void>;
/**
* @param {string} text
* @returns {Promise<void>}
*/
export function read_message(text: string): Promise<void>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly console_error_panic_hook_set: () => void;
  readonly collapse: (a: number, b: number) => number;
  readonly update_nav: () => number;
  readonly load_gzip: (a: number, b: number) => number;
  readonly load_md: (a: number, b: number) => number;
  readonly load_style: () => number;
  readonly update_from_hash: () => number;
  readonly rs_onload: () => number;
  readonly clicked_scroll: () => number;
  readonly show_nav: () => number;
  readonly show_content: () => number;
  readonly search_results: (a: number, b: number) => number;
  readonly search_results_big: (a: number, b: number) => number;
  readonly back_arrow: () => number;
  readonly forward_arrow: () => number;
  readonly on_finish_animation: () => number;
  readonly startle_bird: () => number;
  readonly update_bird_target_location: () => number;
  readonly read_message: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly wasm_bindgen__convert__closures__invoke1_mut__hab830eaad86031a3: (a: number, b: number, c: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h68423c55ff3db5ca: (a: number, b: number, c: number, d: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
