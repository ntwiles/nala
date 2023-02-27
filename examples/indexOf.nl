
enum IndexOfStringResult {
    Found(Number),
    NotFound,
}

func indexOfString(haystack: Array<String>, needle: String ): IndexOfStringResult {
    mut i = 0;

    for item in haystack {
        if (item == needle) {
            break(IndexOfStringResult::Found(i));
        }

        i = i + 1;
    }

    IndexOfStringResult::NotFound;
}

const names = [ 'Jimmy', 'Kim', 'Howard', 'Lalo' ];
const result = indexOfString(names, 'Howard');
print(result); // TODO: This proves that the right result is found, but doesn't yet let us extract it.
