/* Usage for http requests in their current state.
 *
 * TODO: These three `options` fields (method, url, body) are currently the only ones available. 
 * There's no way of setting headers yet, for example.
 *
 * struct HttpResult<T> {
 *    statusCode: Option<String>,
 *    body: Option<T>,
 * }
 */

struct HouseHead {
    firstName: String,
    lastName: String,
} 

struct HouseInfo {
    name: String,
    heads: Array<HouseHead>,
    ghost: String,
    founder: String,
}

func printHouseInfo(house: HouseInfo): Void {
    print('House: ' + house.name);
    print('Founder: ' + house.founder);
    print('Ghost: ' + house.ghost);
    print('Heads:');
    
    for head in house.heads {
        print(' - ' + head.firstName + ' ' + head.lastName);
    }

    print('');
}

func doRequest(url: String): Void {
    const result: HttpResult<Array<HouseInfo>> = http({
        method: 'GET',
        url: url,
    });

    match (result.statusCode) {
        Option::Some(statusCode) => { 
            print('Result Status: ' + statusCode); 

            match (result.body) {
                Option::Some(body) => {
                    print('');

                    for house in body {
                        printHouseInfo(house);
                    }
                }
                Option::None => {
                    print('No body in response.');
                }
            }
        }
        Option::None => { print('Could not complete request.'); }
    }
}

print('Making GET request with (hopeful) success response...');
doRequest('https://wizard-world-api.herokuapp.com/Houses');
print('----------------------------------------------');

print('Making GET request with (hopeful) error response...');
doRequest('https://wizard-world-api.herokuapp.com/GiveMeAnError');
print('----------------------------------------------');

print('Making unsuccessful GET request...');
doRequest('https://foo.bar/baz');