struct Message {
    firstWord: String,
    second: {
        word: String,
    },
    punctuation: Array<String>
}

func callbackA() {
    print('callbackA called!');
}

func callbackB() {
    print('callbackB called!');
}

// This will not be valid Nala once user-defined types are implemented.
func writeMessage(message: Message) {
    print(message.firstWord + ' ' + message.second.word + message.punctuation[0]);
    message.callback(); // TODO: Nothing prevents us from invoking this yet even though callback() isn't in type Message
}

const message = {
    firstWord: 'hello',
    second: {
        word: 'world'
    },
    punctuation: ['!'],
    callback: callbackA // TODO: This causes an error when passing `message` as an arg, even though it shouldn't.
};

writeMessage(message);

message.firstWord = 'this';
message.second.word = 'too';
message.punctuation[0] = '?';
message.callback = callbackB;

writeMessage(message);


