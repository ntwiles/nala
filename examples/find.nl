struct Character {
    name: String,
    alias: Option<String>
}

// TODO: Make this a builtin function.
func find<T>(fn: Func<T, Bool>, list: Array<T>): Option<T> {
    for item in list {
        if (fn(item)) {
            break(Option::Some(item));
        }
    }

    Option::None;
}

func isHeisenberg(character: Character): Bool {
    character.alias == Option::Some('Heisenberg');
}

const characters = [ 
    { name: 'Walter White', alias: Option::Some('Heisenberg') }, 
    { name: 'Jesse Pinkman', alias: Option::Some('Capn Cook') },
    { name: 'Gus Fring', alias: Option::None }, 
];

const maybeHeisenberg = find(isHeisenberg, characters);

match (maybeHeisenberg) {
    Option::Some(heisenberg) => { print('Found Heisenberg: ' + heisenberg.name); } 
    Option::None => { print('Did not find Heisenberg.'); }
}