Nala is an interpreted, expression-oriented programming language with terse syntax designed for quick use in prototyping console applications and in manipulating data.

## Features

- [x] Basic console i/o.
- [x] Lexical scope.
- [x] Conditional branching.
- [x] Arrays and loops.
- [x] Functions (first-class citizens).
- [x] Strict nomitive type system.
- [x] Object literals and member access.
- [x] Patterns (first-class-citizens).
- [ ] Filesystem / Network IO. (Started)
- [ ] Garbage collection.
- [ ] Generics.
- [x] Enum types with data.
- [ ] Error recovery.
- [ ] Function pipelines.
- [ ] Async / await.
- [ ] Module and package systems.

## Usage

Parse and interpret in the console any `.nl` file with the following command:

```
cargo run path/to/script.nl
```

### Examples

Example scripts are provided in the `examples/` directory. 

Within that directory, `sandbox.nl` will be ignored by git.
## Documentation

Documentation can be found on the [Nala Wiki](https://github.com/ntwiles/nala-rust/wiki).

## Syntax Highlighting

Syntax highlighting is available in the form of a VS Code extension [here](https://github.com/ntwiles/nala-vscode-extension).
