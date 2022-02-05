
func printArray(array: Array<IPrint>, label: String) {
    print(label + ':');
    for item in array {
        print(item);
    }
    print('');
}

func bubbleSort(items: Array<ICompare>) {
    mut i = 0;
    mut changed = false;

    for current in items {
        if (i > 0) {
            const prev = items[i - 1];
            
            if (prev > current) {
                items[i] = prev;
                items[i - 1] = current;
                changed = true;
            }
        }

        i = i + 1;
    }

    if (changed) { bubbleSort(items); }
    items;
}

const unsorted = [3, 5, 1, 4, 2];
const sorted = bubbleSort(unsorted);
printArray(sorted, 'Sorted Numbers');