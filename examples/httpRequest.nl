/* Usage for http requests in their current state.
 *
 * TODO: These three `options` fields (method, url, body) are currently the only ones available. 
 * There's no way of setting headers yet, for example.
 */

struct HouseHead {
    firstName: String,
    lastName: String,
} 

struct HouseInfo {
    name: String,
    mascot: String,
    heads: Array<HouseHead>,
    houseGhost: String,
    founder: String,
}

struct Result {
    statusCode: String,
    body: Array<HouseInfo>,
}

func printHouseInfo(house: HouseInfo): Void {
    print('House: ' + house.name);
    print('Founder: ' + house.founder);

    print('Heads:');
    
    for head in house.heads {
        print(' - ' + head.firstName + ' ' + head.lastName);
    }

    print('');
}

print('Making GET request...');

const result = http({
    method: 'GET',
    url: 'https://wizard-world-api.herokuapp.com/Houses',
});


print('Result Status: ' + result.statusCode);
print('');

for house in result.body {
    // TODO: This is erroring right now, find out why.
    printHouseInfo(house);
}