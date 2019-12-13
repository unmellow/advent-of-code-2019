/* tslint:disable */
/**
*/
export function init(): void;
export enum ExecutionState {
  Running,
  Halted,
  NeedsInput,
}
/**
*/
/**
*/
export class ThirteenGame {
  free(): void;
/**
* @param {any} canvas 
* @param {boolean} auto_play 
* @returns {ThirteenGame} 
*/
  constructor(canvas: any, auto_play: boolean);
/**
* @param {number} user_input 
* @returns {number} 
*/
  step(user_input: number): number;
/**
*/
  render_game(): void;
/**
* @returns {number} 
*/
  score(): number;
}

/**
* If `module_or_path` is {RequestInfo}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {RequestInfo | BufferSource | WebAssembly.Module} module_or_path
*
* @returns {Promise<any>}
*/
export default function init (module_or_path: RequestInfo | BufferSource | WebAssembly.Module): Promise<any>;
        