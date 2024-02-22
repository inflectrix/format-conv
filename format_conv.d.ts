/* tslint:disable */
/* eslint-disable */
/**
* @param {string} s
* @returns {any}
*/
export function des_json(s: string): any;
/**
* @param {string} s
* @returns {any}
*/
export function des_toml(s: string): any;
/**
* @param {string} s
* @returns {any}
*/
export function des_ron(s: string): any;
/**
* @param {string} s
* @returns {any}
*/
export function des_yaml(s: string): any;
/**
* @param {string} s
* @returns {any}
*/
export function des_qs(s: string): any;
/**
* @param {string} s
* @returns {any}
*/
export function des_lexpr(s: string): any;
/**
* @param {any} d
* @returns {any}
*/
export function convert_to_all(d: any): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly des_json: (a: number, b: number, c: number) => void;
  readonly des_toml: (a: number, b: number, c: number) => void;
  readonly des_ron: (a: number, b: number, c: number) => void;
  readonly des_yaml: (a: number, b: number, c: number) => void;
  readonly des_qs: (a: number, b: number, c: number) => void;
  readonly des_lexpr: (a: number, b: number, c: number) => void;
  readonly convert_to_all: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
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
