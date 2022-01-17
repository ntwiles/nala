# Introduction

Nala is an interpreted, expression-oriented programming language with terse syntax designed for quick use in prototyping console applications and in manipulating data.

# Features

- [x] Basic console i/o.
- [x] Arithmetic operations.
- [x] Lexical scope.
- [x] Conditional branching.
- [x] Arrays and loops.
- [] Functions.

# Usage

Parse and interpret in the console any `.nl` file with the following command:

```
cargo run path/to/script.nl
```

Example scripts are provided in the `example/` directory.

# Documentation
## Variable Declaration and Assignment
### Constants

Constant variables can be declared with the following syntax:

```
const foo = 7;
print foo;
```

As with the above example, a trailing semicolon is required after both statements.

Values declared with the `const` keyword are immutable (though further clarification to this may be needed when objects are implemented).

### Mutables

Mutable variables are declared similarly:

```
mut foo = 7;
print foo;
foo = 8;
print foo;
```

Both mutable and immutable variables must be initialized with value. The following is not valid nala:

```
mut foo;
foo = 7;
```

### Scope

Lexical scope works as it does in most languages and should be intuitive. Blocks create new scopes in which local bindings can be created The following will throw a runtime error at execution of the last line:

```
if (2 == 2) {
    const foo = 'bar';
}

print foo;
```

### Shadowing

Identifiers can be 'shadowed' in lower scopes by re-using names. The following is valid and will print first `hello`, then `world`:

```
const foo = 'world';

if (2 == 2) {
    const foo = 'hello';
    print foo;
}

print foo;
```

## Expressions

Everything is an expression in Nala, including "statements" like assignment operations. Some expressions resolve 
to the `Void` type though, which cannot be assigned.

### Arithmatic

Arithmetic expressions can be performed between number types:

```
print 5 + 10 * 2 / 4 - 3;
```

Operations will evaluate in DOMA (Delimiter, Order, Multiplicatives, Additives) order, though at present only the following arithmetic operators are implemented (`+`, `-`, `*`, `/`) and delimiter grouping is not yet supported.

### String Concatenation

```
const message = 'hello ' + 'world';
print message;
```

The `+` operator can also be used between strings to perform concatenation.

```
const foo = 'bar';
print foo == 'bar';
print 7 == 8;
```

## Conditional Branching

The syntax for `if` branching should be familiar:

```
if (2 == 2) {
    print 'should print';
}

if (2 == 3) {
    print 'should not print';
}
```

### Conditional Expressions

The following operators are supported for comparisons: `<` `>` `==`:

```
const isGreater = 3 > 2;
const isLesser = 2 < 3;
const isEqual = 2 == 2;
```

## Console Input and Output

### Input
#### Reading String Input

The `read` keyword will get input from the console, treated as a String type.

```
print 'Please enter your name'
const name = read;
print 'Hello ' + name;
```

Note: Once functions are introduced, this keyword will be replaced with an inbuilt function: `read();`
#### Reading Numeric Input

The `readnum` keyword will get input from the console and parse it as a `Num` type.

```
print 'Please enter a number:';
const input = readnum;
print 'The product of your number and 7 is:';
print input * 7;
```

Note: Once functions are implemented, this keyword will be replaced with an inbuilt function: `readnum();`

### Output

Output can be printed to the console with the following syntax:

```
print 'foo';
```

In this code snippet, whitespace is insignificant, strings are surrounded by single quotes ('), and the required trailing semicolon signifies the end of a statement.

Note: Once functions are introduced, this keyword will be replaced with an inbuilt function: `print('foo');`
## Functions 

Functions are declared in this way:

```
func example() {

}
```

And can be invoked thusly:

```
example();
```

Functions return the `Void` type by default, or otherwise will return the resolved value of the first expression 
which does not resolve to type `Void`.

Note: Functions do not yet support parameters.
## Arrays

Arrays are initialized with the following literal syntax:

```
const array = [5, 4, 2, 8];
```

Values can be accessed by index with the following syntax:

```
print array[0];
```

### Iterating Arrays

Arrays can be looped over with `for .. in` syntax:

```
const nums = [5, 4, 2, 8];

for num in nums {
    print num;
}
```