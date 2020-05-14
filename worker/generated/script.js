let wasm_bindgen;
(function() {
    const __exports = {};
    let wasm;

    let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

    cachedTextDecoder.decode();

    let cachegetUint8Memory0 = null;
    function getUint8Memory0() {
        if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory0;
    }

    function getStringFromWasm0(ptr, len) {
        return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
    }

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

    function passStringToWasm0(arg, malloc, realloc) {

        if (realloc === undefined) {
            const buf = cachedTextEncoder.encode(arg);
            const ptr = malloc(buf.length);
            getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
            WASM_VECTOR_LEN = buf.length;
            return ptr;
        }

        let len = arg.length;
        let ptr = malloc(len);

        const mem = getUint8Memory0();

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
            ptr = realloc(ptr, len, len = offset + arg.length * 3);
            const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
            const ret = encodeString(arg, view);

            offset += ret.written;
        }

        WASM_VECTOR_LEN = offset;
        return ptr;
    }

    function _assertClass(instance, klass) {
        if (!(instance instanceof klass)) {
            throw new Error(`expected instance of ${klass.name}`);
        }
        return instance.ptr;
    }

    let cachegetInt32Memory0 = null;
    function getInt32Memory0() {
        if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
            cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
        }
        return cachegetInt32Memory0;
    }
    /**
    */
    class Action {

        static __wrap(ptr) {
            const obj = Object.create(Action.prototype);
            obj.ptr = ptr;

            return obj;
        }

        free() {
            const ptr = this.ptr;
            this.ptr = 0;

            wasm.__wbg_action_free(ptr);
        }
        /**
        * @param {string} action_serialized
        */
        constructor(action_serialized) {
            var ptr0 = passStringToWasm0(action_serialized, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            var ret = wasm.action_new(ptr0, len0);
            return Action.__wrap(ret);
        }
        /**
        * @param {number} response_status_code
        * @returns {number}
        */
        get_status_code(response_status_code) {
            var ret = wasm.action_get_status_code(this.ptr, response_status_code);
            return ret;
        }
        /**
        * @param {HeaderMap} headers
        * @param {number} response_status_code
        * @param {boolean} add_rule_ids_header
        * @returns {HeaderMap}
        */
        filter_headers(headers, response_status_code, add_rule_ids_header) {
            _assertClass(headers, HeaderMap);
            var ptr0 = headers.ptr;
            headers.ptr = 0;
            var ret = wasm.action_filter_headers(this.ptr, ptr0, response_status_code, add_rule_ids_header);
            return HeaderMap.__wrap(ret);
        }
        /**
        * @param {number} response_status_code
        * @returns {BodyFilter}
        */
        create_body_filter(response_status_code) {
            var ret = wasm.action_create_body_filter(this.ptr, response_status_code);
            return BodyFilter.__wrap(ret);
        }
    }
    __exports.Action = Action;
    /**
    */
    class BodyFilter {

        static __wrap(ptr) {
            const obj = Object.create(BodyFilter.prototype);
            obj.ptr = ptr;

            return obj;
        }

        free() {
            const ptr = this.ptr;
            this.ptr = 0;

            wasm.__wbg_bodyfilter_free(ptr);
        }
        /**
        * @returns {boolean}
        */
        is_null() {
            var ret = wasm.bodyfilter_is_null(this.ptr);
            return ret !== 0;
        }
        /**
        * @param {string} body
        * @returns {string}
        */
        filter(body) {
            try {
                var ptr0 = passStringToWasm0(body, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                var len0 = WASM_VECTOR_LEN;
                wasm.bodyfilter_filter(8, this.ptr, ptr0, len0);
                var r0 = getInt32Memory0()[8 / 4 + 0];
                var r1 = getInt32Memory0()[8 / 4 + 1];
                return getStringFromWasm0(r0, r1);
            } finally {
                wasm.__wbindgen_free(r0, r1);
            }
        }
        /**
        * @returns {string}
        */
        end() {
            try {
                wasm.bodyfilter_end(8, this.ptr);
                var r0 = getInt32Memory0()[8 / 4 + 0];
                var r1 = getInt32Memory0()[8 / 4 + 1];
                return getStringFromWasm0(r0, r1);
            } finally {
                wasm.__wbindgen_free(r0, r1);
            }
        }
    }
    __exports.BodyFilter = BodyFilter;
    /**
    */
    class HeaderMap {

        static __wrap(ptr) {
            const obj = Object.create(HeaderMap.prototype);
            obj.ptr = ptr;

            return obj;
        }

        free() {
            const ptr = this.ptr;
            this.ptr = 0;

            wasm.__wbg_headermap_free(ptr);
        }
        /**
        */
        constructor() {
            var ret = wasm.headermap_new();
            return HeaderMap.__wrap(ret);
        }
        /**
        * @param {string} name
        * @param {string} value
        */
        add_header(name, value) {
            var ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            var ptr1 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            wasm.headermap_add_header(this.ptr, ptr0, len0, ptr1, len1);
        }
        /**
        * @returns {number}
        */
        len() {
            var ret = wasm.headermap_len(this.ptr);
            return ret >>> 0;
        }
        /**
        * @returns {boolean}
        */
        is_empty() {
            var ret = wasm.headermap_is_empty(this.ptr);
            return ret !== 0;
        }
        /**
        * @param {number} index
        * @returns {string}
        */
        get_header_name(index) {
            try {
                wasm.headermap_get_header_name(8, this.ptr, index);
                var r0 = getInt32Memory0()[8 / 4 + 0];
                var r1 = getInt32Memory0()[8 / 4 + 1];
                return getStringFromWasm0(r0, r1);
            } finally {
                wasm.__wbindgen_free(r0, r1);
            }
        }
        /**
        * @param {number} index
        * @returns {string}
        */
        get_header_value(index) {
            try {
                wasm.headermap_get_header_value(8, this.ptr, index);
                var r0 = getInt32Memory0()[8 / 4 + 0];
                var r1 = getInt32Memory0()[8 / 4 + 1];
                return getStringFromWasm0(r0, r1);
            } finally {
                wasm.__wbindgen_free(r0, r1);
            }
        }
    }
    __exports.HeaderMap = HeaderMap;
    /**
    */
    class Request {

        static __wrap(ptr) {
            const obj = Object.create(Request.prototype);
            obj.ptr = ptr;

            return obj;
        }

        free() {
            const ptr = this.ptr;
            this.ptr = 0;

            wasm.__wbg_request_free(ptr);
        }
        /**
        * @param {string} uri
        * @param {string} host
        * @param {string} scheme
        * @param {string} method
        */
        constructor(uri, host, scheme, method) {
            var ptr0 = passStringToWasm0(uri, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            var ptr1 = passStringToWasm0(host, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            var ptr2 = passStringToWasm0(scheme, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len2 = WASM_VECTOR_LEN;
            var ptr3 = passStringToWasm0(method, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len3 = WASM_VECTOR_LEN;
            var ret = wasm.request_new(ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
            return Request.__wrap(ret);
        }
        /**
        * @param {string} name
        * @param {string} value
        */
        add_header(name, value) {
            var ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            var ptr1 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            wasm.request_add_header(this.ptr, ptr0, len0, ptr1, len1);
        }
        /**
        * @returns {string}
        */
        serialize() {
            try {
                wasm.request_serialize(8, this.ptr);
                var r0 = getInt32Memory0()[8 / 4 + 0];
                var r1 = getInt32Memory0()[8 / 4 + 1];
                return getStringFromWasm0(r0, r1);
            } finally {
                wasm.__wbindgen_free(r0, r1);
            }
        }
    }
    __exports.Request = Request;

    async function load(module, imports) {
        if (typeof Response === 'function' && module instanceof Response) {

            if (typeof WebAssembly.instantiateStreaming === 'function') {
                try {
                    return await WebAssembly.instantiateStreaming(module, imports);

                } catch (e) {
                    if (module.headers.get('Content-Type') != 'application/wasm') {
                        console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                    } else {
                        throw e;
                    }
                }
            }

            const bytes = await module.arrayBuffer();
            return await WebAssembly.instantiate(bytes, imports);

        } else {

            const instance = await WebAssembly.instantiate(module, imports);

            if (instance instanceof WebAssembly.Instance) {
                return { instance, module };

            } else {
                return instance;
            }
        }
    }

    async function init(input) {
        if (typeof input === 'undefined') {
            let src;
            if (typeof document === 'undefined') {
                src = location.href;
            } else {
                src = document.currentScript.src;
            }
            input = src.replace(/\.js$/, '_bg.wasm');
        }
        const imports = {};
        imports.wbg = {};
        imports.wbg.__wbindgen_throw = function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        };

        if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
            input = fetch(input);
        }

        const { instance, module } = await load(await input, imports);

        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;

        return wasm;
    }

    wasm_bindgen = Object.assign(init, __exports);

})();
 /* redirection.io options */
const options = {
    token: "TOKEN",
    timeout: 1000,
    add_rule_ids_header: false,
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
    const redirectionioRequest = new libredirectionio.Request(urlObject.pathname, urlObject.host, urlObject.protocol.includes('https') ? 'https' : 'http', request.method);

    for (const pair of request.headers.entries()) {
        redirectionioRequest.add_header(pair[0], pair[1]);
    }

    try {
        const agentResponse = await Promise.race([
            fetch('https://agent.redirection.tech/' + options.token + '/action', {
                method: 'POST',
                body: redirectionioRequest.serialize(),
                headers: {
                    'User-Agent': 'cloudflare-service-worker/0.1.0'
                },
            }),
            new Promise((_, reject) =>
                setTimeout(() => reject(new Error('Timeout')), options.timeout)
            ),
        ]);

        const actionStr = await agentResponse.text();

        if (actionStr === "") {
            return [await fetch(request), null];
        }

        const action = new libredirectionio.Action(actionStr);
        const statusCodeBeforeResponse = action.get_status_code(0);

        let response = null;

        if (statusCodeBeforeResponse === 0) {
            response = await fetch(request);
        } else {
            response = new Response('', {
                status: Number(statusCodeBeforeResponse),
            });
        }

        const statusCodeAfterResponse = action.get_status_code(response.status);

        if (statusCodeAfterResponse !== 0) {
            response.status = Number(statusCodeAfterResponse);
        }

        const headerMap = new libredirectionio.HeaderMap();

        for (const pair of response.headers.entries()) {
            headerMap.add_header(pair[0], pair[1]);
        }

        const newHeaderMap = action.filter_headers(headerMap, response.status, options.add_rule_ids_header);
        const newHeaders = new Headers();

        for (let i = 0; i < newHeaderMap.len(); i++) {
            newHeaders.append(newHeaderMap.get_header_name(i), newHeaderMap.get_header_value(i));
        }

        response = new Response(
            response.body, {
                status: response.status,
                statusText: response.statusText,
                headers: newHeaders,
            }
        );

        const bodyFilter = action.create_body_filter(response.status);

        // Skip body filtering
        if (bodyFilter.is_null()) {
            return [response, action];
        }

        const { readable, writable } = new TransformStream();

        filter_body(response.body, writable, bodyFilter);

        return [new Response(readable, response), action];
    } catch (error) {
        return [await fetch(request), null]
    }
}

async function filter_body(readable, writable, bodyFilter) {
    let writer = writable.getWriter();
    let reader = readable.getReader();
    const decoder = new TextDecoder("utf-8");
    const encoder = new TextEncoder("utf-8");
    let data = await reader.read();

    while (!data.done) {
        const chunk = decoder.decode(data.value);
        const filteredData = bodyFilter.filter(chunk);

        if (filteredData) {
            await writer.write(encoder.encode(filteredData));
        }

        data = await reader.read();
    }

    const lastData = bodyFilter.end();

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
