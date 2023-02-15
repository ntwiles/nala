
// TODO: We can set the return type to String here and no error will occur.
func indexOfString(haystack: Array<String>, needle: String ): Number {
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

print(names[index]);
