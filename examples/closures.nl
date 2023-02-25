func closureTestA(): Void {
    const message = 'closures work!';

    func innerFunc(): Void {
        print(message);
    }

    innerFunc();
}

func closureTestB(): Func<Void> {
    const message = 'closures REALLY work!';

    func innerFunc(): Void {
        print(message);
    }

    innerFunc;
}

closureTestA();

// TODO: This fails. Maybe a func should store the scopeId in which it was declared.
// TODO: Write closure unit tests.
// const funcToCall = closureTestB();
// funcToCall();
