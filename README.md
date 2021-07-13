# Introduction

Nala is (read: will be) an interpreted, expression-oriented programming language with terse syntax designed for quick use in prototyping console applications and in manipulating data.

# Features

- [x] Basic console output.
- [x] Arithmetic operations.
- [x] Lexical scope.
- [ ] Conditional branching.
- [ ] Basic console input.
- [ ] Functions

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

As with the above example, a trailing semicolon is required after both statements. Bindings are added to a lexical scope which in effect is global as there's not yet any method of creating new scopes.

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
