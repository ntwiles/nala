const options = {
    method: 'GET',
    url: 'https://reqbin.com/echo/get/json',
    body: 'this is the body'
};

const response = request(options);
print('Got a response: '+ response);