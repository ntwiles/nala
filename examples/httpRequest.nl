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
    heads: Array<HouseHead>,
    ghost: String,
    founder: String,
}

struct Result {
    statusCode: String,
    body: Array<HouseInfo>,
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

print('Making GET request...');

const result = http({
    method: 'GET',
    url: 'https://wizard-world-api.herokuapp.com/Houses',
});


print('Result Status: ' + result.statusCode);
print('');

for house in result.body {
    // TODO: Is seems as though this `house` value doesn't always get checked for 
    // fit against the `HouseInfo` type. This seems to somehow change per run.
    printHouseInfo(house);
}