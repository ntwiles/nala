![Last Commit](https://img.shields.io/github/last-commit/ntwiles/nala)
![File Count](https://img.shields.io/github/directory-file-count/ntwiles/nala)

Nala is an expression-oriented general purpose interpreted programming language designed with ease of use for 
functional programmers in mind.


## Features
- Structural dynamic type system
- Type inference
- Sum types (via enums)
- Generic types and functions
- Pattern matching
- Syntax highlighting (via VS Code extension)

## Roadmap
- Migrate to static type system
- Network I/O (Started)
- Filesystem I/O
- Error recovery
- Automatic function currying
- Function pipeline support?
- Async / await support
- Garbage collection
- Module and package systems
- String interpolation
- Full language support via LSP server.

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

## Editor Support

Syntax highlighting is available in the form of a VS Code extension [here](https://github.com/ntwiles/nala-vscode-extension). Full environment agnostic language support in the form of an LSP language server is underway.

## Known Issues

Comments are stripped before parsing in a preprocessing stage. This stage uses regex patterns to match both 
single-line (`//`) and multi-line (`/* */`) comments, which are imperfect. Embedding comment sequences inside 
strings (for example) will result in a portion of the string being stripped during preprocessing which in turn 
will cause a parse error.

## Questions / Contact

You can find my contact information on my [github profile](https://github.com/ntwiles), please feel free to get in touch regarding Nala.
