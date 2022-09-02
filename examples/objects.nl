
func callbackA() {
    print('callbackA called!');
}

func callbackB() {
    print('callbackB called!');
}

// Objects can not yet be accepted as function arguments as they cannot yet be typed.
func writeMessage() {
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

writeMessage();

message.firstWord = 'this';
message.second.word = 'too';
message.punctuation[0] = '?';
message.callback = callbackB;

writeMessage();


