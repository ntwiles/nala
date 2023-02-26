/** TODO: We're using this struct because we don't yet support data in enum variants.
 * Once we do, use the following:
 *
 * enum Result {
 *     Some(Number),
 *     None,
 * }
 */

struct Result {
    success: Bool,
    index: Number,
}


func indexOfString(haystack: Array<String>, needle: String ): Result {
    mut i = 0;

    for item in haystack {
        if (item == needle) {
            break({ success: true, index: i });
        }

        i = i + 1;
    }

    { success: false, index: 0 };
}

const names = [ 'Jimmy', 'Kim', 'Howard', 'Lalo' ];
const result = indexOfString(names, 'Howard');

if (result.success) {
    print(names[result.index]);
}