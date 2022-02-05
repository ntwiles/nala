
func indexOf(haystack: Array<IEqual>, needle: IEqual ) {
    mut i = 0;
    for item in haystack {
        if (item == needle) {
            break(i);
        }

        i = i + 1;
    }

    -1;
}

const names = [ 'Nathan', 'Sam', 'Patrick', 'Liz' ];
const numbers = [ 42, 69, 420 ];

const indexOfString = indexOf(names, 'Patrick');
const indexOfNum = indexOf(numbers, 420);

print(indexOfString);
print(indexOfNum);