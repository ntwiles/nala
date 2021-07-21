const secret = 52;
const attempts = [0, 1, 2];

print 'Guess the secret number!';

mut won = 1 == 2;

for i in attempts {
    const guess = readnum;
    print 'You guessed: ' + guess;

    if (guess == secret) {
        print 'You guessed right!';
        won = 1 == 1;
    }

    if (guess > secret) {
        print 'You guessed too high!';
    }

    if (guess < secret) {
        print 'You guessed too low!';
    }

    const guessesLeft = 2 - i;
    print 'You have ' + guessesLeft + ' guesses left.';
}

const false = 1 == 2;
if (won == false) {
    print 'Sorry, you ran out of tries!';
}