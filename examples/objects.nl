struct Message {
    firstWord: String,
    second: {
        word: String,
    },
    punctuation: Array<String>,
    callback: Func,
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
    message.callback(); 
    // print(message.extra); // TODO: Nothing prevents us from invoking this yet even though callback() isn't in type Message
}

const message = {
    firstWord: 'hello',
    second: {
        word: 'world'
    },
    punctuation: ['!'],
    callback: callbackA,
    //extra: 7 // TODO: This causes an error when passing `extra` as an arg since it's not on Message, even though it shouldn't be a problem.
};

writeMessage(message);

message.firstWord = 'this';
message.second.word = 'too';
message.punctuation[0] = '?';
message.callback = callbackB;

writeMessage(message);


