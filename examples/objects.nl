struct Message {
    firstWord: string,
    // second: {
    //     word: string,
    // },
    // punctuation: Array<string>
}

func callbackA() {
    print('callbackA called!');
}

func callbackB() {
    print('callbackB called!');
}

// This will not be valid Nala once user-defined types are implemented.
func writeMessage(message: Object) {
    print(message.firstWord + ' ' + message.second.word + message.punctuation[0]);
    message.callback();
}

const message = {
    firstWord: 'hello',
    second: {
        word: 'world'
    },
    punctuation: ['!'],
    callback: callbackA
};

writeMessage(message);

message.firstWord = 'this';
message.second.word = 'too';
message.punctuation[0] = '?';
message.callback = callbackB;

writeMessage(message);


