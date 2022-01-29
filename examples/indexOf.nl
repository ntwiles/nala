

func indexOf(haystack: Array<String>, needle: String ) {
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

const indexOfPatrick = indexOf(names, 'Patrick');
const indexOfJay = indexOf(names, 'Jay');

print(indexOfPatrick);
print(indexOfJay);