/* attaching the event listener */
addEventListener('fetch', event => {
    event.passThroughOnException();
    event.respondWith(redirectionio_fetch(event.request, event));
});

async function redirectionio_fetch(request, event) {
    const options = {
        token: REDIRECTIONIO_TOKEN || null,
        timeout: parseInt(REDIRECTIONIO_TIMEOUT, 10),
        add_rule_ids_header: REDIRECTIONIO_ADD_HEADER_RULE_IDS === 'true',
        version: REDIRECTIONIO_VERSION || 'redirection-io-cloudflare/dev',
        instance_name: REDIRECTIONIO_INSTANCE_NAME || 'undefined',
    }

    if (options.token === null) {
        return await fetch(request);
    }

    const libredirectionio = wasm_bindgen;
    await wasm_bindgen(wasm);

    const redirectionioRequest = create_redirectionio_request(request);
    const [action, registerCachePromise] = await get_action(request, redirectionioRequest);
    const response = await proxy(request, redirectionioRequest, action, libredirectionio, options);
    const clientIP = request.headers.get("CF-Connecting-IP");

    event.waitUntil(async function () {
        await registerCachePromise;
        await log(response, redirectionioRequest, action, libredirectionio, options, clientIP);
    }());

    return response;
}

function create_redirectionio_request(request) {
    const urlObject = new URL(request.url);
    const redirectionioRequest = new libredirectionio.Request(urlObject.pathname + urlObject.search, urlObject.host, urlObject.protocol.includes('https') ? 'https' : 'http', request.method);

    for (const pair of request.headers.entries()) {
        redirectionioRequest.add_header(pair[0], pair[1]);
    }

    return redirectionioRequest;
}

async function get_action(request, redirectionioRequest) {
    const cache = caches.default;
    const cacheUrl = new URL(request.url)
    cacheUrl.pathname = "/get-action/" + redirectionioRequest.get_hash().toString()

    // Convert to a GET to be able to cache
    const cacheKey = new Request(cacheUrl.toString(), {
        headers: request.headers,
        method: "GET",
    });

    let actionStr = await cache.match(cacheKey);

    if (!actionStr) {
        const response = await Promise.race([
            fetch('https://agent.redirection.io/' + options.token + '/action', {
                method: 'POST',
                body: redirectionioRequest.serialize().toString(),
                headers: {
                    'User-Agent': 'cloudflare-worker/' + options.version,
                    'x-redirectionio-instance-name': options.instance_name,
                },
            }),
            new Promise((_, reject) =>
                setTimeout(() => reject(new Error('Timeout')), options.timeout)
            ),
        ]);

        actionStr = await response.text();
    }

    const registerCachePromise = cache.put(cacheKey, actionStr);

    if (actionStr === "") {
        return [libredirectionio.Action.empty(), registerCachePromise]
    }

    try {
        return [new libredirectionio.Action(actionStr), registerCachePromise];
    } catch (e) {
        console.error(e);

        return [new libredirectionio.Action.empty(), registerCachePromise];
    }
}

/* Redirection.io logic */
async function proxy(request, redirectionioRequest, action, libredirectionio, options) {

    try {
        const statusCodeBeforeResponse = action.get_status_code(0);

        let response;

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

        response = new Response(response.body, {
            status: response.status,
            statusText: response.statusText,
            headers: newHeaders,
        });

        const bodyFilter = action.create_body_filter(response.status);

        // Skip body filtering
        if (bodyFilter.is_null()) {
            return response;
        }

        const { readable, writable } = new TransformStream();

        filter_body(response.body, writable, bodyFilter);

        return new Response(readable, response);
    } catch (err) {
        console.error(err);

        return await fetch(request);
    }
}

async function filter_body(readable, writable, bodyFilter) {
    let writer = writable.getWriter();
    let reader = readable.getReader();
    let data = await reader.read();

    while (!data.done) {
        const filteredData = bodyFilter.filter(data.value);

        if (filteredData) {
            await writer.write(filteredData);
        }

        data = await reader.read();
    }

    const lastData = bodyFilter.end();

    if (lastData) {
        await writer.write(lastData);
    }

    await writer.close();
}

async function log(response, redirectionioRequest, action, libredirectionio, options, clientIP) {
    if (response === null) {
        return;
    }

    const timestamp = Date.now();
    const responseHeaderMap = new libredirectionio.HeaderMap();

    for (const pair of response.headers.entries()) {
        responseHeaderMap.add_header(pair[0], pair[1]);
    }

    try {
        const logAsJson = libredirectionio.create_log_in_json(
            redirectionioRequest,
            response.status,
            responseHeaderMap,
            action,
            'cloudflare-worker/' + options.version,
            BigInt(timestamp),
            clientIP,
        );

        return await fetch(
            'https://agent.redirection.io/' + options.token + '/log',
            {
                method: 'POST',
                body: logAsJson,
                headers: {
                    'User-Agent': 'cloudflare-worker/' + options.version,
                    'x-redirectionio-instance-name': options.instance_name,
                },
            }
        );
    } catch (err) {
        console.error(err);
    }
}
