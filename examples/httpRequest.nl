
const options = {
    method: 'GET',
    url: 'https://reqbin.com/echo/get/json'
};

const response = request(options);
print('Got a response: '+ response);