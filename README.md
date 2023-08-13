# BEM Parser

A Rust-based parser for the BEM (Block Element Modifier) file format, utilizing the [Pest](https://github.com/pest-parser/pest) parsing library.

[![Rust](https://github.com/vortex-design/bem/actions/workflows/rust.yml/badge.svg)](https://github.com/vortex-design/bem/actions/workflows/rust.yml)

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

### Installation

Add `bem` to your `Cargo.toml`:

```toml
[dependencies]
bem = "0.2.0"
```

### Usage

Here's a simple example to parse a BEM formatted string:

```rust
use bem::parse_bem;

let input = "block[mod1,mod2]\nelement1\nelement2[mod3]";
let bem_block = parse_bem(input).unwrap();

// You can now access `bem_block.name`, `bem_block.modifiers`, and `bem_block.elements`.
```

## Error Handling

The `parse_bem` function returns a `Result<BEMBlock, String>`, allowing you to handle parsing errors explicitly. Here's an example:

```rust
let input = "block[mod1,mod2]\nelement1\nelement2[mod3]";
match parse_bem(input) {
    Ok(bem_block) => {
        // Process the parsed block
    },
    Err(error) => {
        println!("Failed to parse BEM content: {}", error);
    }
}
```

## Contributing

Feel free to open issues or pull requests if you have suggestions, improvements, or fixes.

## License

MIT License. See [LICENSE](LICENSE) for details.
