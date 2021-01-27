/* attaching the event listener */
addEventListener('fetch', event => {
    event.respondWith(handle(event.request));
});

async function handle(request) {
    try {
        return await redirectionio_fetch(request)
    } catch (err) {
        console.error(err);

        // Display the error stack.
        return await fetch(request);
    }
}

async function redirectionio_fetch(request) {
    const options = {
        token: REDIRECTIONIO_TOKEN || null,
        timeout: parseInt(REDIRECTIONIO_TIMEOUT, 10),
        add_rule_ids_header: REDIRECTIONIO_ADD_HEADER_RULE_IDS === 'true',
        version: REDIRECTIONIO_VERSION || 'dev',
        instance_name: REDIRECTIONIO_INSTANCE_NAME || 'undefined',
    }

    if (options.token === null) {
        return await fetch(request);
    }

    const libredirectionio = wasm_bindgen;
    await wasm_bindgen(wasm);
    const [response, redirectionioRequest, action] = await proxy(request, libredirectionio, options);
    const clientIP = request.headers.get("CF-Connecting-IP");

    await log(response, redirectionioRequest, action, libredirectionio, options, clientIP);

    return response;
}

/* Redirection.io logic */
async function proxy(request, libredirectionio, options) {
    const urlObject = new URL(request.url);
    const redirectionioRequest = new libredirectionio.Request(urlObject.pathname + urlObject.search, urlObject.host, urlObject.protocol.includes('https') ? 'https' : 'http', request.method);
    let action = libredirectionio.Action.empty();

    for (const pair of request.headers.entries()) {
        redirectionioRequest.add_header(pair[0], pair[1]);
    }

    try {
        const requestSerialized = redirectionioRequest.serialize();
        const agentResponse = await Promise.race([
            fetch('https://agent.redirection.io/' + options.token + '/action', {
                method: 'POST',
                body: requestSerialized.toString(),
                headers: {
                    'User-Agent': 'cloudflare-worker/' + options.version,
                    'x-redirectionio-instance-name': options.instance_name,
                },
            }),
            new Promise((_, reject) =>
                setTimeout(() => reject(new Error('Timeout')), options.timeout)
            ),
        ]);

        const actionStr = await agentResponse.text();

        if (actionStr === "") {
            return [await fetch(request), redirectionioRequest, action];
        }

        action = new libredirectionio.Action(actionStr);
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
            return [response, redirectionioRequest, action];
        }

        const { readable, writable } = new TransformStream();

        filter_body(response.body, writable, bodyFilter);

        return [new Response(readable, response), redirectionioRequest, action];
    } catch (err) {
        console.error(err);

        return [await fetch(request), redirectionioRequest, action]
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
