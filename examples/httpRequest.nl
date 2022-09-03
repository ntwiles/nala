func test() {
    print('test');
}

const foo = 'test'; // comment

const options = {
    method: 'GET',
    url: '',
    body: ''
};

const result = http(options);
print(result.statusCode);