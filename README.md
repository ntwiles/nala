# Introduction

Nala is (read: will be) an interpreted programming language with terse syntax designed for quick use in prototyping console applications and in manipulating data.

# Features

- [x] Basic console output.
- [x] Arithmetic operations.
- [x] Lexical scope.
- [ ] Functions

# Documentation

## `print` keyword

Output can be printed to the console with the following syntax:
`print 'foo';`
In this code snippet, whitespace is insignificant, strings are surrounded by single quotes ('), and the required trailing semicolon signifies the end of a statement.

```
print 'hello world';
print 10 * 2 / 4 + 5 - 3;
```

The above shows support for chaining multiple statements, and also for complex arithmetic operations which evaluate in the expected order.

## `const` keyword

Variables can be declared with the following syntax:
`const foo;`
As with the above example, a trailing semicolon is required. The variable initially holds a null value. Assignment at the same time as declaration is not yet supported. Bindings are added to a lexical scope which in effect is global as there's not yet any method of creating new scopes.
