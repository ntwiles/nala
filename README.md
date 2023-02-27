![Last Commit](https://img.shields.io/github/last-commit/ntwiles/nala)
![File Count](https://img.shields.io/github/directory-file-count/ntwiles/nala)

Nala is an interpreted, expression-oriented programming language with terse syntax designed for quick use in prototyping console applications and in manipulating data.

## Features

- [x] Basic console I/O.
- [x] Lexical scope
- [x] Conditional branching
- [x] Arrays and loops
- [x] Functions (first-class citizens)
- [x] Structural type system
- [x] Object literals and member access
- [x] Comments (single-line, multi-line)
- [ ] Enum sum types (Started)
- [ ] Filesystem / Network I/O (Started)
- [ ] Generics
- [ ] Error recovery
- [ ] Pattern matching
- [ ] Function pipelines
- [ ] Async / await
- [ ] Garbage collection
- [ ] Module and package systems
- [ ] String interpolation

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

## Known Issues

Comments are stripped before parsing in a preprocessing stage. This stage uses regex patterns to match both 
single-line (`//`) and multi-line (`/* */`) comments, which are imperfect. Embedding comment sequences inside 
strings (for example) will result in a portion of the string being stripped during preprocessing which in turn 
will cause a parse error.
