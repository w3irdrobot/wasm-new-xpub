/* tslint:disable */
/* eslint-disable */
/**
*/
export class XPub {
  free(): void;
/**
* @returns {XPub}
*/
  static new(): XPub;
/**
* @returns {string}
*/
  mnemonic(): string;
/**
* @returns {string}
*/
  xpub(): string;
/**
* @returns {string}
*/
  fingerprint(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_xpub_free: (a: number) => void;
  readonly xpub_new: () => number;
  readonly xpub_mnemonic: (a: number, b: number) => void;
  readonly xpub_xpub: (a: number, b: number) => void;
  readonly xpub_fingerprint: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
