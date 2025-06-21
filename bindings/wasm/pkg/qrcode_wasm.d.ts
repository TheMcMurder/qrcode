/* tslint:disable */
/* eslint-disable */
export function hello(): string;
/**
 * Renders a QR code as SVG and returns the result as a string
 */
export function render_qr_svg(url: string, config?: QrConfig | null): string;
/**
 * Renders a QR code as PNG and returns the result as a Uint8Array
 */
export function render_qr_png(url: string, config?: QrConfig | null): string;
/**
 * Renders a QR code as JPEG and returns the result as a Uint8Array
 */
export function render_qr_jpeg(url: string, config?: QrConfig | null): string;
/**
 * Returns the dimensions of a QR code for a given URL
 */
export function get_qr_dimensions(url: string, config?: QrConfig | null): Uint32Array;
export class QrConfig {
  free(): void;
  constructor(finder_shape: string, data_shape: string, finder_color: string, data_color: string);
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly hello: () => [number, number];
  readonly __wbg_qrconfig_free: (a: number, b: number) => void;
  readonly qrconfig_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
  readonly render_qr_svg: (a: number, b: number, c: number) => [number, number];
  readonly render_qr_png: (a: number, b: number, c: number) => [number, number, number, number];
  readonly render_qr_jpeg: (a: number, b: number, c: number) => [number, number, number, number];
  readonly get_qr_dimensions: (a: number, b: number, c: number) => [number, number];
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
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
