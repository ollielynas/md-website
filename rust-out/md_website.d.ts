/* tslint:disable */
/* eslint-disable */
/**
*/
export function console_error_panic_hook_set(): void;
/**
* @param {string} path
*/
export function collapse(path: string): void;
/**
* todo: make this function return a result
* @returns {boolean | undefined}
*/
export function update_nav(): boolean | undefined;
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

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly console_error_panic_hook_set: () => void;
  readonly collapse: (a: number, b: number) => void;
  readonly update_nav: () => number;
  readonly load_gzip: (a: number, b: number) => number;
  readonly load_md: (a: number, b: number) => number;
  readonly load_style: () => number;
  readonly update_from_hash: () => number;
  readonly rs_onload: () => number;
  readonly clicked_scroll: () => number;
  readonly show_nav: () => number;
  readonly show_content: () => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hd0a5cb23812feaef: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h991d9838853e4706: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
