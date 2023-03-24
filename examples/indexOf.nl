// TODO: Consider making this a builtin function.
func indexOf<T>(haystack: Array<String>, needle: T ): Option<Number> {
    mut i = 0;

    for item in haystack {
        if (item == needle) {
            break(Option::Some(i));
        }

        i = i + 1;
    }

    Option::None;
}

func makeSearchForName(names: Array<String>): Func<String, Void> {
    func searchForName(name: String): Void {
        match (indexOf(names, name)) {
            Option::Some(index) => { 
                print('Name "' + name + '" Found: ' + names[index] + '!'); 
            }
            Option::None => { print('Name "' + name + '" not found!'); }
        }
    }

    searchForName;
}

const searchForName = makeSearchForName([ 'Jimmy', 'Kim', 'Howard', 'Lalo' ]);

searchForName('Howard');
searchForName('Mike');