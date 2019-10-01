(function() {
    const __exports = {};
    let wasm;

    let WASM_VECTOR_LEN = 0;

    let cachedTextEncoder = new TextEncoder('utf-8');

    const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
        ? function (arg, view) {
        return cachedTextEncoder.encodeInto(arg, view);
    }
        : function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    });

    let cachegetUint8Memory = null;
    function getUint8Memory() {
        if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory;
    }

    function passStringToWasm(arg) {

        let len = arg.length;
        let ptr = wasm.__wbindgen_malloc(len);

        const mem = getUint8Memory();

        let offset = 0;

        for (; offset < len; offset++) {
            const code = arg.charCodeAt(offset);
            if (code > 0x7F) break;
            mem[ptr + offset] = code;
        }

        if (offset !== len) {
            if (offset !== 0) {
                arg = arg.slice(offset);
            }
            ptr = wasm.__wbindgen_realloc(ptr, len, len = offset + arg.length * 3);
            const view = getUint8Memory().subarray(ptr + offset, ptr + len);
            const ret = encodeString(arg, view);

            offset += ret.written;
        }

        WASM_VECTOR_LEN = offset;
        return ptr;
    }

    const u32CvtShim = new Uint32Array(2);

    const uint64CvtShim = new BigUint64Array(u32CvtShim.buffer);

    let cachegetUint32Memory = null;
    function getUint32Memory() {
        if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
            cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
        }
        return cachegetUint32Memory;
    }

    let cachegetInt32Memory = null;
    function getInt32Memory() {
        if (cachegetInt32Memory === null || cachegetInt32Memory.buffer !== wasm.memory.buffer) {
            cachegetInt32Memory = new Int32Array(wasm.memory.buffer);
        }
        return cachegetInt32Memory;
    }

    let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

    function getStringFromWasm(ptr, len) {
        return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
    }
    /**
    * @param {string} project_id
    * @param {string} rules_data
    * @param {BigInt} cache_limit
    * @returns {string}
    */
    __exports.update_rules_for_router = function(project_id, rules_data, cache_limit) {
        const retptr = 8;
        uint64CvtShim[0] = cache_limit;
        const low0 = u32CvtShim[0];
        const high0 = u32CvtShim[1];
        const ret = wasm.update_rules_for_router(retptr, passStringToWasm(project_id), WASM_VECTOR_LEN, passStringToWasm(rules_data), WASM_VECTOR_LEN, low0, high0);
        const memi32 = getInt32Memory();
        const v0 = getStringFromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();
        wasm.__wbindgen_free(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 1);
        return v0;
    };

    /**
    * @param {string} project_id
    * @param {string} url
    * @returns {string}
    */
    __exports.get_rule_for_url = function(project_id, url) {
        const retptr = 8;
        const ret = wasm.get_rule_for_url(retptr, passStringToWasm(project_id), WASM_VECTOR_LEN, passStringToWasm(url), WASM_VECTOR_LEN);
        const memi32 = getInt32Memory();
        let v0;
        if (memi32[retptr / 4 + 0] !== 0) {
            v0 = getStringFromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();
            wasm.__wbindgen_free(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 1);
        }
        return v0;
    };

    /**
    * @param {string} project_id
    * @param {string} url
    * @returns {string}
    */
    __exports.get_trace_for_url = function(project_id, url) {
        const retptr = 8;
        const ret = wasm.get_trace_for_url(retptr, passStringToWasm(project_id), WASM_VECTOR_LEN, passStringToWasm(url), WASM_VECTOR_LEN);
        const memi32 = getInt32Memory();
        let v0;
        if (memi32[retptr / 4 + 0] !== 0) {
            v0 = getStringFromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();
            wasm.__wbindgen_free(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 1);
        }
        return v0;
    };

    /**
    * @param {string} rule_str
    * @param {string} url
    * @param {number} response_code
    * @returns {string}
    */
    __exports.get_redirect = function(rule_str, url, response_code) {
        const retptr = 8;
        const ret = wasm.get_redirect(retptr, passStringToWasm(rule_str), WASM_VECTOR_LEN, passStringToWasm(url), WASM_VECTOR_LEN, response_code);
        const memi32 = getInt32Memory();
        let v0;
        if (memi32[retptr / 4 + 0] !== 0) {
            v0 = getStringFromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();
            wasm.__wbindgen_free(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 1);
        }
        return v0;
    };

    /**
    * @param {string} rule_str
    * @param {string} headers_str
    * @returns {string}
    */
    __exports.header_filter = function(rule_str, headers_str) {
        const retptr = 8;
        const ret = wasm.header_filter(retptr, passStringToWasm(rule_str), WASM_VECTOR_LEN, passStringToWasm(headers_str), WASM_VECTOR_LEN);
        const memi32 = getInt32Memory();
        const v0 = getStringFromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();
        wasm.__wbindgen_free(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 1);
        return v0;
    };

    /**
    * @param {string} rule_str
    * @param {string} filter_id
    * @returns {string}
    */
    __exports.create_body_filter = function(rule_str, filter_id) {
        const retptr = 8;
        const ret = wasm.create_body_filter(retptr, passStringToWasm(rule_str), WASM_VECTOR_LEN, passStringToWasm(filter_id), WASM_VECTOR_LEN);
        const memi32 = getInt32Memory();
        let v0;
        if (memi32[retptr / 4 + 0] !== 0) {
            v0 = getStringFromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();
            wasm.__wbindgen_free(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 1);
        }
        return v0;
    };

    /**
    * @param {string} filter_id
    * @param {string} filter_body
    * @returns {string}
    */
    __exports.body_filter = function(filter_id, filter_body) {
        const retptr = 8;
        const ret = wasm.body_filter(retptr, passStringToWasm(filter_id), WASM_VECTOR_LEN, passStringToWasm(filter_body), WASM_VECTOR_LEN);
        const memi32 = getInt32Memory();
        let v0;
        if (memi32[retptr / 4 + 0] !== 0) {
            v0 = getStringFromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();
            wasm.__wbindgen_free(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 1);
        }
        return v0;
    };

    /**
    * @param {string} filter_id
    * @returns {string}
    */
    __exports.body_filter_end = function(filter_id) {
        const retptr = 8;
        const ret = wasm.body_filter_end(retptr, passStringToWasm(filter_id), WASM_VECTOR_LEN);
        const memi32 = getInt32Memory();
        let v0;
        if (memi32[retptr / 4 + 0] !== 0) {
            v0 = getStringFromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();
            wasm.__wbindgen_free(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 1);
        }
        return v0;
    };

    const heap = new Array(32);

    heap.fill(undefined);

    heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getArrayU8FromWasm(ptr, len) {
    return getUint8Memory().subarray(ptr / 1, ptr / 1 + len);
}

function init(module) {

    let result;
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_new_59cb74e423758ede = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_558ba5917b466edd = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ret0 = passStringToWasm(ret);
        const ret1 = WASM_VECTOR_LEN;
        getInt32Memory()[arg0 / 4 + 0] = ret0;
        getInt32Memory()[arg0 / 4 + 1] = ret1;
    };
    imports.wbg.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
        const v0 = getStringFromWasm(arg0, arg1).slice();
        wasm.__wbindgen_free(arg0, arg1 * 1);
        console.error(v0);
    };
    imports.wbg.__wbg_new_3a746f2619705add = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_call_f54d3a6dadb199ca = function(arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
        const ret = getObject(arg0) === getObject(arg1);
        return ret;
    };
    imports.wbg.__wbg_self_ac379e780a0d8b94 = function(arg0) {
        const ret = getObject(arg0).self;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_crypto_1e4302b85d4f64a2 = function(arg0) {
        const ret = getObject(arg0).crypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getRandomValues_1b4ba144162a5c9e = function(arg0) {
        const ret = getObject(arg0).getRandomValues;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_require_6461b1e9a0d7c34a = function(arg0, arg1) {
        const ret = require(getStringFromWasm(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_randomFillSync_1b52c8482374c55b = function(arg0, arg1, arg2) {
        getObject(arg0).randomFillSync(getArrayU8FromWasm(arg1, arg2));
    };
    imports.wbg.__wbg_getRandomValues_1ef11e888e5228e9 = function(arg0, arg1, arg2) {
        getObject(arg0).getRandomValues(getArrayU8FromWasm(arg1, arg2));
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm(arg0, arg1));
    };

    if ((typeof URL === 'function' && module instanceof URL) || typeof module === 'string' || (typeof Request === 'function' && module instanceof Request)) {

        const response = fetch(module);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                return response
                .then(r => {
                    if (r.headers.get('Content-Type') != 'application/wasm') {
                        console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                        return r.arrayBuffer();
                    } else {
                        throw e;
                    }
                })
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module, imports)
        .then(result => {
            if (result instanceof WebAssembly.Instance) {
                return { instance: result, module };
            } else {
                return result;
            }
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;

        return wasm;
    });
}

self.wasm_bindgen = Object.assign(init, __exports);

})();
 /* redirection.io options */
const options = {
    token: YOUR_OWN_TOKEN,
    timeout: 2000,
};

/* attaching the event listener */
addEventListener('fetch', async event => {
    try {
        event.respondWith(respondWithCallback(event.request));
    } catch (e) {
        event.respondWith(event.request);
    }
});

async function respondWithCallback(request) {
    const libredirectionio = wasm_bindgen;
    await wasm_bindgen(wasm);
    const [response, rule] = await handle(request, libredirectionio);

    await log(request, response, rule);

    return response;
}

/* Redirection.io logic */
async function handle(request, libredirectionio) {
    const urlObject = new URL(request.url);
    const context = {
        host: urlObject.host,
        request_uri: urlObject.pathname,
        user_agent: request.headers.get('user-agent'),
        referer: request.headers.get('referer'),
        scheme: urlObject.protocol.includes('https') ? 'https' : 'http',
        use_json: true,
    };

    try {
        const response = await Promise.race([
            fetch('https://proxy.redirection.io/' + options.token + '/match-rule', {
                method: 'POST',
                body: JSON.stringify(context),
                headers: {
                    'User-Agent': 'cloudflare-service-worker/0.1.0'
                },
            }),
            new Promise((_, reject) =>
                setTimeout(() => reject(new Error('Timeout')), options.timeout)
            ),
        ]);

        const ruleStr = await response.text();

        if (ruleStr === "") {
            return [await fetch(request), null];
        }

        const rule = JSON.parse(ruleStr);

        // No rule matching
        if (rule.id === "") {
            return [await fetch(request), null];
        }

        // Get redirect when no response
        const redirectStr = libredirectionio.get_redirect(ruleStr, request.url, 0);

        if (redirectStr) {
            const redirect = JSON.parse(redirectStr);

            if (redirect.status_code !== 0) {
                return [
                    new Response('', {
                        status: Number(redirect.status_code),
                        headers: {
                            Location: redirect.location,
                        },
                    }),
                    rule
                ];
            }
        }

        let backendResponse = await fetch(request);
        const redirectAfterResponseStr = libredirectionio.get_redirect(ruleStr, request.url, backendResponse.status);

        if (redirectAfterResponseStr) {
            const redirectAfterResponse = JSON.parse(redirectAfterResponseStr);

            // Get redirect with response
            if (redirectAfterResponse.status_code !== 0) {
                return [
                    new Response('', {
                        status: Number(redirect.status_code),
                        headers: {
                            Location: redirect.location,
                        },
                    }),
                    rule
                ];
            }
        }

        const headers = [];

        for (const pair of backendResponse.headers.entries()) {
            headers.push({
                name: pair[0],
                value: pair[1],
            });
        }

        const newHeadersStr = libredirectionio.header_filter(ruleStr, JSON.stringify(headers));

        if (newHeadersStr && newHeadersStr !== "") {
            const newHeaders = JSON.parse(newHeadersStr);
            const newHeadersObject = new Headers();

            for (const newHeader of newHeaders) {
                newHeadersObject.append(newHeader.name, newHeader.value);
            }

            newHeadersObject.append("X-RedirectionIO-Rule", rule.id);

            backendResponse = new Response(
                backendResponse.body, {
                    status: backendResponse.status,
                    statusText: backendResponse.statusText,
                    headers: newHeadersObject,
                }
            );
        }

        const filterBodyId = libredirectionio.create_body_filter(ruleStr, "filter_id");

        if (filterBodyId && filterBodyId !== "") {
            let { readable, writable } = new TransformStream();

            filter_body(backendResponse.body, writable, filterBodyId, libredirectionio);

            return [new Response(readable, backendResponse), rule];
        }

        return [backendResponse, rule];
    } catch (error) {
        return [await fetch(request), null]
    }
}

async function filter_body(readable, writable, filterBodyId, libredirectionio) {
    let writer = writable.getWriter();
    let reader = readable.getReader();
    const decoder = new TextDecoder("utf-8");
    const encoder = new TextEncoder("utf-8");
    let data = await reader.read();

    while (!data.done) {
        const chunk = decoder.decode(data.value);
        const filteredData = libredirectionio.body_filter(filterBodyId, chunk);

        if (filteredData) {
            await writer.write(encoder.encode(filteredData));
        }

        data = await reader.read();
    }

    const lastData = libredirectionio.body_filter_end(filterBodyId);

    if (lastData) {
        await writer.write(encoder.encode(lastData));
    }

    await writer.close();
}

async function log(request, response, rule) {
    if (response === null) {
        return;
    }

    const urlObject = new URL(request.url);
    const context = {
        status_code: response.status,
        host: urlObject.host,
        method: request.method,
        request_uri: urlObject.pathname,
        user_agent: request.headers.get('user-agent'),
        referer: request.headers.get('referer'),
        scheme: urlObject.protocol.includes('https') ? 'https' : 'http',
        use_json: true,
    };

    if (response.headers.get('Location')) {
        context.target = response.headers.get('Location');
    }

    if (rule !== null) {
        context.rule_id = rule.id;
    }

    try {
        return await fetch(
            'https://proxy.redirection.io/' + options.token + '/log',
            {
                method: 'POST',
                body: JSON.stringify(context),
                headers: {
                    'User-Agent': 'cloudflare-service-worker/0.1.0'
                },
            }
        );
    } catch (error) {
        // Do nothing, do not matters if some logs are in errors
        console.log('could not log');
        console.log(error)
    }
}
