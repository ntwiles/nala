func test() {
    print('test');
}

/* block
comment
*/

const options = {
    method: 'GET', // this is a comment
    url: 'https://reqbin.com/echo',
    body: ''
};

const result = http(options);
print(result.statusCode);