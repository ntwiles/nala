
enum Comparison {
    LessThan,
    Equal,
    GreaterThan
}

func compare(a: ICompare, b: ICompare) {
    if (a < b) {
        Comparison::LessThan;
    }

    if (a > b) {
        Comparison::GreaterThan;
    }

    Comparison::Equal;
}

func bubbleSort(items: Array<Number>, comparator: Func<Number, Number>) {
    mut i = 0;
    mut changed = false;

    for current in items {
        if (i > 0) {
            const prev = items[i - 1];
            
            if (comparator(prev, current) == Comparison::GreaterThan) {
                items[i] = prev;
                items[i - 1] = current;
                changed = true;
            }
        }

        i = i + 1;
    }

    if (changed) {
        bubbleSort(items, comparator);
    }

    items;
}

func printArray(array: Array<IPrint>, label: String) {
    print(label + ':');
    for item in array {
        print(item);
    }
    print('');
}

const unsortedNums = [3, 5, 1, 4, 2];
printArray(unsortedNums, 'Unsorted Number Array');

const sortedNums = bubbleSort(unsortedNums, compare);
printArray(sortedNums, 'Sorted Number Array');