const secret = 52;
print 'Guess the secret number!';
const guess = readnum;
print 'You guessed: ' + guess;

if (guess == secret) {
    print 'You guessed right!';
}

if (guess > secret) {
    print 'You guessed too high!';
}

if (guess < secret) {
    print 'You guessed too low!';
}