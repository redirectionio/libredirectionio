export * from './wasm/redirectionio';

export function init(instantiate?: () => Promise<WebAssembly.Instance>): Promise<void>;
