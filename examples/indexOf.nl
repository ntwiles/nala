
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

pattern some = Option::Some($);

if (result is some) {
    const index = unwrap result as some;
    print(index);
}