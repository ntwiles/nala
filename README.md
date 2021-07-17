# Introduction

Nala is (read: will be) an interpreted, expression-oriented programming language with terse syntax designed for quick use in prototyping console applications and in manipulating data.

# Features

- [x] Basic console i/o.
- [x] Arithmetic operations.
- [x] Lexical scope.
- [x] Conditional branching.
- [ ] Loops and functions.

# Documentation

## `print` keyword

Output can be printed to the console with the following syntax:

```
print 'foo';
```

In this code snippet, whitespace is insignificant, strings are surrounded by single quotes ('), and the required trailing semicolon signifies the end of a statement.

## `const` keyword

Variables can be declared with the following syntax:

```
const foo = 7;
print foo;
```

As with the above example, a trailing semicolon is required after both statements.

Values declared with the `const` keyword are immutable (though further clarification to this may be needed when objects are implemented). A future `let` keyword will allow for mutable variables to be declared.

## Expressions

Arithmetic expressions can be performed between number types:

```
print 5 + 10 * 2 / 4 - 3;
```

Operations will evaluate in DOMA (Delimiter, Order, Multiplicatives, Additives) order, though at present only the following arithmetic operators are implemented (`+`, `-`, `*`, `/`) and delimiter grouping is not yet supported.

```
const message = 'hello ' + 'world';
print message;
```

The `+` operator can also be used between strings to perform concatenation.

```
const foo = "bar";
print foo == "bar";
print 7 == 8;
```

## `if` keyword

The most basic of conditional branching is implemented in nala, as with most other languages, with `if` statements. The syntax should be familiar to everyone:

```
if (2 == 2) {
    print 'should print';
}

if (2 == 3) {
    print 'should not print';
}
```

## Scope

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
