func test() {
    print('test');
}

const getRequest = {
    method: 'GET',
    url: 'https://httpbin.org/get',
};

const postRequest = {
    method: 'POST',
    url: 'https://httpbin.org/post',
    body: 'test',
};

print('Making GET request...');
mut result = http(getRequest);
print(result.statusCode);

print('Making POST request...');
result = http(postRequest);
print(result.statusCode);