# BEM Parser

A Rust-based parser for the BEM (Block Element Modifier) file format, utilizing the [Pest](https://github.com/pest-parser/pest) parsing library.

[![Rust](https://github.com/vortex-design/bem/actions/workflows/rust.yml/badge.svg)](https://github.com/vortex-design/bem/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/mickmao/bem/branch/main/graph/badge.svg?token=AWSB407T0R)](https://codecov.io/gh/mickmao/bem)
[![Release-plz](https://github.com/vortex-design/bem/actions/workflows/release-plz.yml/badge.svg)](https://github.com/vortex-design/bem/actions/workflows/release-plz.yml)
[![CD](https://github.com/vortex-design/bem/actions/workflows/CD.yml/badge.svg)](https://github.com/vortex-design/bem/actions/workflows/CD.yml)

## Overview

This parser provides functionality to parse BEM notations, primarily used in CSS methodologies for naming classes in HTML. With this parser, you can interpret and work with BEM notations programmatically in Rust.

## Features

- Parse BEM blocks, elements, and modifiers.
- Support for dashes in block and element names.
- Support for enclosing modifiers in square brackets and separating with commas.
- Robust error handling with detailed parsing error messages.
- Lightweight and efficient parsing using Pest.

## Getting Started

### Prerequisites

Ensure you have Rust and Cargo installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Usage

Here's a simple example to parse a BEM formatted string:

```rust
use bem::parse;

let input = "block[mod1,mod2]\nelement1\nelement2[mod3]";
let bem_block = parse(input).unwrap();

// You can now access `bem_block.name`, `bem_block.modifiers`, and `bem_block.elements`.
```

## Error Handling

The `parse` function returns a `Result<BEMBlock, String>`, allowing you to handle parsing errors explicitly. Here's an example:

```rust
let input = "block[mod1,mod2]\nelement1\nelement2[mod3]";
match parse(input) {
    Ok(bem_block) => {
        // Process the parsed block
    },
    Err(error) => {
        println!("Failed to parse BEM content: {}", error);
    }
}
```

## Documentation

Find the documentation for your installed version at <https://docs.rs/bem/VERSION/bem/>, replacing `VERSION` with your installed version number.

## Contributing

Feel free to open issues or pull requests if you have suggestions, improvements, or fixes.

## License

MIT License. See [LICENSE](LICENSE) for details.
