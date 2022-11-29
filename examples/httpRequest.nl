/* Usage for http requests in their current state.
 *
 * NOTE: These three `options` fields are currently the only ones available. 
 * There's no way of setting headers yet, for example.
 */

print('Making GET request...');

mut result = http({
    method: 'GET',
    url: 'https://httpbin.org/get',
    body: '',
});

print(result.statusCode);

print('Making POST request...');

result = http({
    method: 'POST',
    url: 'https://httpbin.org/post',
    body: 'test',
});

print(result.statusCode);