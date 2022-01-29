
func bubbleSort(items: Array<Number>, comparator: Func) {
    wiles (isUnsorted(items, comparator)) {
        items = bubblePass(items, comparator);
    }

    items;
}

func isUnsorted(items: Array<Number>, comparator: Func) {
    mut i = 0; 
    for current in items {
        if (i > 0) {
            const prev = items[i - 1];

            if (comparator(prev, current) < 0) {
                break(true);
            }
        }

        i = i + 1;
    }

    false;
}

func bubblePass(items: Array<Number>, comparator: Func) {
    mut i = 0;
    for current in items {
        if (i > 0) {
            const prev = items[i - 1];
            
            if (comparator(prev, current) < 0) {
                items[i] = prev;
                items[i-1] = current;
            }
        }

        i = i + 1;
    }

    items;
}

func byValue(a: Number, b: Number) {
    b - a;
}

const unsorted = [ 3, 5, 1, 4, 2];
const sorted = bubbleSort(unsorted, byValue);

print('sorted');

for item in sorted {
    print(item);
}