/* Usage for http requests in their current state.
 *
 * NOTE: These three `options` fields (method, url, body) are currently the only ones available. 
 * There's no way of setting headers yet, for example.
 */


// TODO: Make this a builtin type.
struct Result {
    statusCode: String,
    body: {
        origin: String,
    },
}

func printResult(result: Result): Void {
    print('Status Code: ' + result.statusCode);
    print('Response Origin: ' + result.body.origin);
    print('');
}

print('Making GET request...');

const resultA = http({
    method: 'GET',
    url: 'https://httpbin.org/get',
});

printResult(resultA);

print('Making POST request...');

const resultB = http({
    method: 'POST',
    url: 'https://httpbin.org/post',
});

printResult(resultB);