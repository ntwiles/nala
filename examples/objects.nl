struct Message {
    firstWord: String,
    second: {
        word: String,
    },
    punctuation: Array<String>,
}

func callbackA(): Void {
    print('callbackA called!');
}

func callbackB(): Void {
    print('callbackB called!');
}

// This will not be valid Nala once user-defined types are implemented.
func writeMessage(message: Message): Void {
    print(message.firstWord + ' ' + message.second.word + message.punctuation[0]);
    message.callback(); // TODO: Nothing prevents us from invoking this yet even though callback() isn't in type Message
}

const message = {
    firstWord: 'hello',
    second: {
        word: 'world'
    },
    punctuation: ['!'],
    callback: callbackA,
    extraField: 0
};

writeMessage(message);

message.firstWord = 'this';
message.second.word = 'too';
message.punctuation[0] = '?';
message.callback = callbackB;

writeMessage(message);


