# BEM Specification

## General Format

- The BEM structure is composed of a single block followed by zero or more elements.
- Each block and element can optionally have modifiers.
- The entire structure must occupy the full input without any unexpected or trailing characters or sequences.

## Block

- A block starts at the beginning of the input.
- It is composed of a name optionally followed by modifiers.
- The name must start with a lowercase ASCII alphabetic character and can be followed by any combination of ASCII alphanumeric characters. Single dashes are allowed between characters but cannot start or end the name.
- Modifiers for a block are enclosed in parentheses and separated by the pipe (`|`) character. Each modifier also adheres to the naming conventions of blocks.

## Element

- Each element starts on a new line.
- Like blocks, an element is composed of a name optionally followed by modifiers.
- The naming convention for elements is the same as blocks: it starts with a lowercase ASCII alphabetic character, followed by any combination of ASCII alphanumeric characters. Single dashes are permitted between characters but not at the beginning or end.
- Modifiers for an element are structured in the same way as those for blocks.

## Examples

### media-player.bem

```
media-player(dark)
button(fast-forward|rewind)
timeline
```

- `media-player(dark)`- Represents a block named `media-player` with one modifier: `dark`.
- `button(fast-forward|rewind)` on a new line after a block represents an element named `button` with two modifiers: `fast-forward` and `rewind`.
- `timeline` on another line represents another element named `timeline` with no modifiers.

## Constraints

- The input must start with a block.
- Names cannot start or end with dashes and cannot have consecutive dashes.
- Modifiers are optional for both blocks and elements.
- The entire structure must fit the provided input. There shouldn't be any characters or sequences after the last element.