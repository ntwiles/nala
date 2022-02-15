
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

const names = [ 'Jimmy', 'Kim', 'Howard', 'Lalo' ];

const result = indexOf(names, 'Howard');

if (result is Option::Some($)) {
    const index = unwrap result as Option::Some(_);
    print(index);
}

if (result is Option::Some(_)) {
    print('Found it! Who cares where.');
}