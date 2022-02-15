
enum Option {
    Some(Number),
    None
}

func indexOf(haystack: Array<IEqual>, needle: IEqual ) {
    mut i = 0;
    for item in haystack {
        if (item == needle) {
            break(Option::Some(i));
        }

        i = i + 1;
    }

    Option::None;
}

const names = [ 'Nathan', 'Sam', 'Patrick', 'Liz' ];
const numbers = [ 42, 69, 420 ];

const indexOfString = indexOf(names, 'Jay');
const indexOfNum = indexOf(numbers, 420);

print(indexOfString);
print(indexOfNum);