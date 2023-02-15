func outerFunc(): Void {
    const testA = 'closures work!';
    const testB = 'closures REALLY work!';

    func innerFuncA(): Void {
        print(testA);
    }

    func innerFuncB(): Void {
        print(testB);
    }

    innerFuncB;
}

const funcToCall = outerFunc();

// TODO: This fails. Maybe a func should store the scopeId in which it was declared.
funcToCall();
