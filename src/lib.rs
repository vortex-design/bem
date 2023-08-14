//! `bem` is a Rust crate that provides a simple and intuitive interface to work with BEM (Block Element Modifier) notation.
//! It includes functions to parse BEM blocks, elements, and their modifiers, and to convert them to and from JSON representation.
//!
//! With this crate, you can easily serialize and deserialize BEM structures in your Rust applications,
//! making it an excellent choice for web developers, CSS authors, and anyone working with BEM-based design.
//!
//! # Features
//!
//! - **Parse BEM Notation**: Use the `parse` function to interpret BEM strings and create corresponding Rust structures.
//! - **JSON Serialization and Deserialization**: Convert BEM blocks to JSON strings and vice versa with the `to_json` and `from_json` functions.
//! - **Customizable Models**: Work with `BEMBlock` and `BEMElement` structs to represent BEM structures, supporting custom modifiers and elements.
//!
//! # Quick Start
//!
//! Add the crate to your Cargo.toml and start working with BEM structures right away!
//!
//! ```
//! use bem::{BEMBlock, to_json, from_json};
//!
//! let bem_block = BEMBlock { name: "media-player".to_string(), modifiers: vec![], elements: vec![] };
//! let json = to_json(&bem_block).unwrap();
//! let bem_block_from_json = from_json(&json).unwrap();
//! ```
//!
//! Please see the individual function and structure documentation for detailed information and examples.

pub use models::{ BEMBlock, BEMElement };
pub use parser::parse;

mod models;
mod parser;

/// Converts a `BEMBlock` into a JSON string.
///
/// This function takes a reference to a `BEMBlock` and serializes it into a JSON string.
/// It returns a `Result` containing the JSON string if the conversion is successful, or
/// a `serde_json::Error` if there is a problem during serialization.
///
/// # Arguments
///
/// * `bem_block`: &BEMBlock - A reference to the `BEMBlock` to be converted to JSON.
///
/// # Returns
///
/// * `Result<String, serde_json::Error>` - A result containing the JSON string or an error.
///
/// # Examples
///
/// ```
/// use bem::{BEMBlock, to_json};
///
/// let bem_block = BEMBlock { name: "media-player".to_string(), modifiers: vec![], elements: vec![] };
/// let json = to_json(&bem_block).unwrap();
/// ```
pub fn to_json(bem_block: &BEMBlock) -> Result<String, serde_json::Error> {
	let json_output = serde_json::to_string(&bem_block)?;

	Ok(json_output)
}

/// Converts a JSON string into a `BEMBlock`.
///
/// This function takes a JSON string and deserializes it into a `BEMBlock`.
/// It returns a `Result` containing the `BEMBlock` if the conversion is successful, or
/// a `serde_json::Error` if there is a problem during deserialization.
///
/// # Arguments
///
/// * `json`: &str - The JSON string to be converted to a `BEMBlock`.
///
/// # Returns
///
/// * `Result<BEMBlock, serde_json::Error>` - A result containing the `BEMBlock` or an error.
///
/// # Examples
///
/// ```
/// use bem::{BEMBlock, from_json};
///
/// let json = "{\"name\":\"media-player\",\"modifiers\":[],\"elements\":[]}";
/// let bem_block = from_json(json).unwrap();
/// ```
pub fn from_json(json: &str) -> Result<BEMBlock, serde_json::Error> {
	let bem_block = serde_json::from_str(json)?;

	Ok(bem_block)
}

#[cfg(test)]
mod tests {
	use super::{ BEMBlock, BEMElement };

	fn create_test_bem_block() -> BEMBlock {
		BEMBlock {
			name: "media-player".to_string(),
			modifiers: vec!["dark".to_string()],
			elements: vec![
				BEMElement {
					name: "button".to_string(),
					modifiers: vec!["fast-forward".to_string(), "rewind".to_string()],
				},
				BEMElement {
					name: "timeline".to_string(),
					modifiers: vec![],
				}
			],
		}
	}

	#[test]
	fn test_to_json() {
		let bem_block = create_test_bem_block();
		let result = super::to_json(&bem_block);

		assert!(result.is_ok());

		insta::assert_snapshot!(result.unwrap());
	}

	#[test]
	fn test_from_json() {
		let json =
			"{\"name\":\"media-player\",\"modifiers\":[\"dark\"],\"elements\":[{\"name\":\"button\",\"modifiers\":[\"fast-forward\",\"rewind\"]},{\"name\":\"timeline\",\"modifiers\":[]}]}";
		let result = super::from_json(json);

		assert!(result.is_ok());

		assert_eq!(result.unwrap(), create_test_bem_block());
	}
}
