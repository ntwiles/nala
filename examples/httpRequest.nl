/* Usage for http requests in their current state.
 *
 * NOTE: These three `options` fields (method, url, body) are currently the only ones available. 
 * There's no way of setting headers yet, for example.
 */


// TODO: Make a builtin type for http call results.
struct GetResult {
    statusCode: String,
    body: {
        origin: String,
    },
}

struct PostResult {
    statusCode: String,
    body: {
        origin: String,
        data: {
            hello: String,
        },
    },
}

func printGetResult(result: GetResult): Void {
    print('Status Code: ' + result.statusCode);
    print('Response Origin: ' + result.body.origin);
    print('');
}

func printPostResult(result: PostResult): Void {
    print('Status Code: ' + result.statusCode);
    print('Response Origin: ' + result.body.origin);
    print('Response Data: ' + result.body.data);
    print('');
}

print('Making GET request...');

const resultA = http({
    method: 'GET',
    url: 'https://httpbin.org/get',
});

printGetResult(resultA);

print('Making POST request...');

const resultB = http({
    method: 'POST',
    url: 'https://httpbin.org/post',
    body: 'test' // TODO: Right now this will get stripped for some reason if it's an object instead of a string.
});

printPostResult(resultB);