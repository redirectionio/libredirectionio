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
