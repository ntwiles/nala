![Last Commit](https://img.shields.io/github/last-commit/ntwiles/nala)
![File Count](https://img.shields.io/github/directory-file-count/ntwiles/nala)

Note: This project is no longer in development. See `Implementation` and `Known Issues` for more information.

Nala is an expression-oriented (see below) general purpose interpreted programming language, designed with ease 
of use for functional programmers in mind.

## Features
- Structural typing with dynamic type inference
- Sum types (via enums)
- Generic types and functions
- Pattern matching
- Syntax highlighting (via VS Code extension)

## Expression-Orientation
In Nala, everything is an expression that returns a value. Values of unit type `Void` cannot be assigned
to variables.


## Implementation
Nala is dynamically typed and uses a top-down recursive interpreter that operates directly on the
Abstract Syntax Tree (AST). Types are inferred dynamically at runtime, and Nala uses a structural
type system to enforce type equivalence. This means that type compatibility is determined by the 
structure of the types (i.e., the fields or elements they contain), rather than their explicit type names.

There is no separate semantic analysis stage, so many semantic errors (including type errors) are 
caught only at runtime.

## Usage

Parse and interpret in the console any `.nl` file with the following command:

```
cargo run path/to/script.nl
```

### Examples

Example scripts are provided in the [examples](https://github.com/ntwiles/nala/tree/main/examples) directory. 

Within that directory, `sandbox.nl` will be ignored by git.

## Documentation

Documentation can be found on the [Nala Wiki](https://github.com/ntwiles/nala-rust/wiki).

## Editor Support

Syntax highlighting for all Nala constructs is available in the form of a VS Code extension 
[here](https://github.com/ntwiles/nala-vscode-extension).

## Known Issues

### Comments
Comments are stripped before parsing in a preprocessing stage. This stage uses regex patterns to match 
both single-line (`//`) and multi-line (`/* */`) comments, which is inherently imperfect. For instance, 
embedding comment sequences inside strings can result in part of the string being stripped, leading to 
a parse error. This is a known limitation of using regex for comment handling.

### Grammar
There are some fundamental issues with operation precedence in the grammar, which (while not yet encountered)
may result in unpredictable expression resolution in certain cases. Refactoring the grammar to achieve a 
clearer operation hierarchy would necessitate major changes to both the AST and interpreter, contributing 
to the decision to bring this project to a close.

## Contributing / Forking

Although this project is no longer in development, feel free to fork the repository or reach out if you'd 
like to contribute or explore the language further. 

## Questions / Contact

You can find my contact information on my [github profile](https://github.com/ntwiles), please feel free to get in touch regarding Nala.
