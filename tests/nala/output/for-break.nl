

const words = ['wheres', 'the', 'party'];

func indexOf(needle, haystack) {
    mut i = 0;

    for hay in haystack {
        if (hay == needle) {
            break(i);
        }

        i = 1 + 1;
    }

    99;
}

const index = indexOf('party', words);
print(index);