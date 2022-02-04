
func bubbleSort(items: Array<Number>, comparator: Func<Number, Number>) {
    wiles (isUnsorted(items, comparator)) {
        items = bubblePass(items, comparator);
    }

    items;
}

func isUnsorted(items: Array<Number>, comparator: Func<Number, Number>) {
    mut i = 0; 
    for current in items {
        if (i > 0) {
            const prev = items[i - 1];

            if (comparator(prev, current) == Comparison::GreaterThan) {
                break(true);
            }
        }

        i = i + 1;
    }

    false;
}

func bubblePass(items: Array<Number>, comparator: Func<Number, Number>) {
    mut i = 0;
    for current in items {
        if (i > 0) {
            const prev = items[i - 1];
            
            if (comparator(prev, current) == Comparison::GreaterThan) {
                items[i] = prev;
                items[i - 1] = current;
            }
        }

        i = i + 1;
    }

    items;
}

enum Comparison {
    LessThan,
    Equal,
    GreaterThan
}

func compareByValue(a: Number, b: Number) {
    if (a < b) {
        Comparison::LessThan;
    }

    if (a > b) {
        Comparison::GreaterThan;
    }

    Comparison::Equal;
}

func printArray(array: Array<IPrintable>, label: String) {
    print(label + ':');
    for item in array {
        print(item);
    }
    print('');
}

const unsorted = [3, 5, 1, 4, 2];
printArray(unsorted, 'Unsorted');

const sorted = bubbleSort(unsorted, compareByValue);
printArray(sorted, 'Sorted');

