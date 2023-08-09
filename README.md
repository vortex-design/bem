# BEM Parser

A Rust-based parser for the BEM file format, utilizing the [Pest](https://github.com/pest-parser/pest) parsing library.

[![Rust](https://github.com/vortex-design/bem/actions/workflows/rust.yml/badge.svg)](https://github.com/vortex-design/bem/actions/workflows/rust.yml)

## Overview

This parser provides functionality to parse BEM (Block Element Modifier) notations, primarily used in CSS methodologies for naming classes in HTML. With this parser, you can interpret and work with BEM notations programmatically in Rust.

## Features

- Parse BEM blocks, elements, and modifiers.
- Support for single dashes in names.
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
bem = "0.1.0"
```

### Usage

Here's a simple example:

```rust
use bem::BEMParser;

let bem_content = "media-player(dark|light)\nbutton(fast-forward|rewind)\ntimeline";
let parsed = BEMParser::parse(&bem_content).expect("Failed to parse BEM content");

// Process the parsed content...
for pair in parsed {
  println!("{:?}", pair);
}
```

## Contributing

Feel free to open issues or pull requests if you have suggestions, improvements, or fixes.

## License

MIT License. See [LICENSE](LICENSE) for details.
