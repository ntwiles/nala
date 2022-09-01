

func indexOfString(haystack: Array<String>, needle: String ) {
    mut i = 0;

    for item in haystack {
        if (item == needle) {
            break(i);
        }

        i = i + 1;
    }

    -1;
}

const names = [ 'Jimmy', 'Kim', 'Howard', 'Lalo' ];
const index = indexOfString(names, 'Howard');

print(index);
