
enum IndexOfStringResult {
    Found(Number),
    NotFound,
}

func indexOfString(haystack: Array<String>, needle: String ): IndexOfStringResult {
    mut i = 0;

    for item in haystack {
        if (item == needle) {
            break(IndexOfStringResult::Found(i));
        }

        i = i + 1;
    }

    IndexOfStringResult::NotFound;
}

func makeSearchForName(names: Array<String>): Func<String, Void> {
    func searchForName(name: String): Void {
        match (indexOfString(names, name)) {
            IndexOfStringResult::Found(index) => { 
                print('Name "' + name + '" Found: ' + names[index] + '!'); 
            }
            IndexOfStringResult::NotFound => { print('Name "' + name + '" not found!'); }
        }
    }

    searchForName;
}

const searchForName = makeSearchForName([ 'Jimmy', 'Kim', 'Howard', 'Lalo' ]);

searchForName('Howard');
searchForName('Mike');