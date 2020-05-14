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
    const [response, redirectionioRequest, action] = await handle(request, libredirectionio);

    await log(request, response, redirectionioRequest, action, libredirectionio);

    return response;
}

/* Redirection.io logic */
async function handle(request, libredirectionio) {
    const urlObject = new URL(request.url);
    const redirectionioRequest = new libredirectionio.Request(urlObject.pathname, urlObject.host, urlObject.protocol.includes('https') ? 'https' : 'http', request.method);
    let action = libredirectionio.Action.empty();

    for (const pair of request.headers.entries()) {
        redirectionioRequest.add_header(pair[0], pair[1]);
    }

    try {
        const agentResponse = await Promise.race([
            fetch('https://agent.redirection.io/' + options.token + '/action', {
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
    } catch (error) {
        return [await fetch(request), redirectionioRequest, action]
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

async function log(request, response, redirectionioRequest, action, libredirectionio) {
    if (response === null) {
        return;
    }

    const timestamp = Date.now();
    const responseHeaderMap = new libredirectionio.HeaderMap();

    for (const pair of response.headers.entries()) {
        responseHeaderMap.add_header(pair[0], pair[1]);
    }

    const log = libredirectionio.create_log_in_json(
        redirectionioRequest,
        response.status,
        responseHeaderMap,
        action,
        "cloudflare-service-worker/0.1.0",
        BigInt(timestamp),
    );

    console.log(log);

    try {
        return await fetch(
            'https://agent.redirection.io/' + options.token + '/log',
            {
                method: 'POST',
                body: log,
                headers: {
                    'User-Agent': 'cloudflare-service-worker/0.1.0'
                },
            }
        );
    } catch (error) {
        console.log(error)
    }
}
