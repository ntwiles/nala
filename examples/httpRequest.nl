/* Usage for http requests in their current state.
 *
 * NOTE: These three `options` fields (method, url, body) are currently the only ones available. 
 * There's no way of setting headers yet, for example.
 */

struct Result {
    statusCode: String,
    body: String,
}

func printResult(result: Result): Void {
    print(result.statusCode);
    print(result.body);
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
    body: 'test',
});

printResult(resultB);