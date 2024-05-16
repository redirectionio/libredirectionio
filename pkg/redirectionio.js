import * as redirectionio from './wasm/redirectionio_bg.js';
import wasmModule from './wasm/redirectionio_bg.wasm?module';

export * from './wasm/redirectionio_bg.js';

function getImports() {
    const imports = {"./redirectionio_bg.js": {}};

    for (const functionName of Object.keys(redirectionio)) {
        if (functionName.startsWith("__") && functionName !== "__wbg_set_wasm") {
            imports["./redirectionio_bg.js"][functionName] = redirectionio[functionName];
        }
    }

    return imports;
}

let loadedModule = null;

export async function init(instantiate) {
    if (loadedModule) {
        return;
    }

    if (!instantiate) {
        instantiate = async () => {
            return await WebAssembly.instantiate(wasmModule, getImports());
        };
    }

    const module = await instantiate();

    redirectionio.__wbg_set_wasm(module.exports);
    loadedModule = module;
    redirectionio.init_log();
}
