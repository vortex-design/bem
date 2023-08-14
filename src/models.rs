//! This module defines the core data structures used throughout the library
//! for representing BEM (Block Element Modifier) components. These structures
//! are used for both parsing and serializing BEM notation.

use serde::{ Serialize, Deserialize };

/// Represents a BEM (Block Element Modifier) block, which consists of a name,
/// a list of modifiers, and a list of elements.
///
/// A BEM block is the top-level abstraction in BEM, and it can have zero or more
/// elements and modifiers associated with it.
///
/// # Example
///
/// ```
/// use bem::BEMBlock;
///
/// let block = BEMBlock {
///     name: "media-player".to_string(),
///     modifiers: vec!["dark".to_string()],
///     elements: vec![/* BEMElement structs go here */],
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BEMBlock {
	pub name: String,
	pub modifiers: Vec<String>,
	pub elements: Vec<BEMElement>,
}

/// Represents an element within a BEM block, with its own name and list of modifiers.
///
/// A BEM element is a component part of a BEM block, and it can have zero or more
/// modifiers associated with it.
///
/// # Example
///
/// ```
/// use bem::BEMElement;
///
/// let element = BEMElement {
///     name: "button".to_string(),
///     modifiers: vec!["fast-forward".to_string(), "rewind".to_string()],
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BEMElement {
	pub name: String,
	pub modifiers: Vec<String>,
}
