use pest::Parser;
use pest_derive::Parser;
use crate::models::{ BEMBlock, BEMElement };

#[derive(Parser)]
#[grammar = "grammar/bem.pest"]
struct BEMGrammar;

/// Parses a BEM (Block, Element, Modifier) syntax string into a structured representation.
///
/// The function uses a Pest grammar to break down the input into a `BEMBlock` that encapsulates the
/// block's name, modifiers, and elements. Elements can also have their own modifiers.
///
/// The expected format for the input string follows the BEM naming convention:
/// - The block name is defined first.
/// - Modifiers are enclosed in square brackets and separated by commas, e.g. `[modifier1,modifier2]`.
/// - Elements are listed on new lines after the block, with their own names and modifiers.
///
/// # Arguments
///
/// * `input`: &str - The input string containing the BEM syntax to be parsed.
///
/// # Returns
///
/// * `Result<BEMBlock, String>` - A result containing the parsed `BEMBlock` structure if parsing was successful,
///   or an error message if there was a problem during parsing.
///
/// # Examples
///
/// The input can be in the following format and should be saved with the `.bem` file extension:
///
/// ```plaintext
/// media-player[dark]
/// button[fast-forward,rewind]
/// timeline
/// ```
///
/// Save this content in a file with a `.bem` extension, like `example.bem`, and then you can parse it with the following code:
///
/// ```
/// use bem::parse;
/// use std::fs;
///
/// let input = fs::read_to_string("example.bem").expect("Failed to read BEM file");
/// let bem_block = parse(&input).unwrap();
///
/// // You can now access `bem_block.name`, `bem_block.modifiers`, and `bem_block.elements`.
pub fn parse(input: &str) -> Result<BEMBlock, String> {
	let mut name = String::new();
	let mut modifiers = Vec::new();
	let mut elements = Vec::new();

	match BEMGrammar::parse(Rule::bem, input) {
		Ok(bem) => {
			for pair in bem.into_iter() {
				match pair.as_rule() {
					Rule::block => {
						let (block_name, block_modifiers) = parse_part(pair).map_err(|e|
							format!("Error parsing block: {}", e)
						)?;
						name = block_name;
						modifiers = block_modifiers;
					}
					Rule::element => {
						let (element_name, element_modifiers) = parse_part(pair).map_err(|e|
							format!("Error parsing element: {}", e)
						)?;
						elements.push(BEMElement {
							name: element_name,
							modifiers: element_modifiers,
						});
					}
					Rule::EOI => {
						break;
					}
					_ => {
						return Err(format!("Unexpected rule: {:?}", pair.as_rule()));
					}
				}
			}
		}
		Err(e) => {
			return Err(format!("Pest parsing error: {}", e));
		}
	}

	Ok(BEMBlock {
		name,
		modifiers,
		elements,
	})
}

fn parse_part(pair: pest::iterators::Pair<Rule>) -> Result<(String, Vec<String>), String> {
	let mut name = String::new();
	let mut modifiers = Vec::new();

	for inner_pair in pair.into_inner() {
		match inner_pair.as_rule() {
			Rule::name => {
				name = inner_pair.as_str().to_string();
			}
			Rule::modifiers => {
				for modifier in inner_pair.into_inner() {
					if let Rule::name = modifier.as_rule() {
						modifiers.push(modifier.as_str().to_string());
					}
				}
			}
			_ => {
				return Err(format!("Unexpected rule: {:?}", inner_pair.as_rule()));
			}
		}
	}

	Ok((name, modifiers))
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_parse_block() {
		let input = "foo";
		let result = super::parse(input);

		assert!(result.is_ok());

		let bem_block = result.unwrap();

		assert_eq!(bem_block, super::BEMBlock {
			name: "foo".to_string(),
			modifiers: vec![],
			elements: vec![],
		});
	}

	#[test]
	fn test_parse_block_with_dashes() {
		let input = "foo-bar-baz";
		let result = super::parse(input);

		assert!(result.is_ok());

		let bem_block = result.unwrap();

		assert_eq!(bem_block, super::BEMBlock {
			name: "foo-bar-baz".to_string(),
			modifiers: vec![],
			elements: vec![],
		});
	}

	#[test]
	fn test_parse_block_with_modifier() {
		let input = "foo[bar]";
		let result = super::parse(input);

		assert!(result.is_ok());

		let bem_block = result.unwrap();

		assert_eq!(bem_block, super::BEMBlock {
			name: "foo".to_string(),
			modifiers: vec!["bar".to_string()],
			elements: vec![],
		});
	}

	#[test]
	fn test_parse_block_with_modifier_with_dashes() {
		let input = "foo[bar-baz-qux]";
		let result = super::parse(input);

		assert!(result.is_ok());

		let bem_block = result.unwrap();

		assert_eq!(bem_block, super::BEMBlock {
			name: "foo".to_string(),
			modifiers: vec!["bar-baz-qux".to_string()],
			elements: vec![],
		});
	}

	#[test]
	fn test_parse_block_with_modifiers() {
		let input = "foo[bar,baz,qux]";
		let result = super::parse(input);

		assert!(result.is_ok());

		let bem_block = result.unwrap();

		assert_eq!(bem_block, super::BEMBlock {
			name: "foo".to_string(),
			modifiers: vec!["bar".to_string(), "baz".to_string(), "qux".to_string()],
			elements: vec![],
		});
	}

	#[test]
	fn test_parse_block_with_modifiers_and_spaces() {
		let input = "foo[  bar  ,  baz  ]";
		let result = super::parse(input);

		assert!(result.is_ok());

		let bem_block = result.unwrap();

		assert_eq!(bem_block, super::BEMBlock {
			name: "foo".to_string(),
			modifiers: vec!["bar".to_string(), "baz".to_string()],
			elements: vec![],
		});
	}

	#[test]
	fn test_parse_block_with_modifiers_and_newlines_and_tabs() {
		let input = "foo[\n\tbar,\n\tbaz]";
		let result = super::parse(input);

		assert!(result.is_ok());

		let bem_block = result.unwrap();

		assert_eq!(bem_block, super::BEMBlock {
			name: "foo".to_string(),
			modifiers: vec!["bar".to_string(), "baz".to_string()],
			elements: vec![],
		});
	}

	#[test]
	fn test_parse_block_with_modifiers_and_trailing_commas() {
		let input = "foo[bar,baz,]";
		let result = super::parse(input);

		assert!(result.is_ok());

		let bem_block = result.unwrap();

		assert_eq!(bem_block, super::BEMBlock {
			name: "foo".to_string(),
			modifiers: vec!["bar".to_string(), "baz".to_string()],
			elements: vec![],
		});
	}

	#[test]
	fn test_parse_block_with_parentheses() {
		let input = "foo(bar,baz)";
		let result = super::parse(input);

		assert!(result.is_err());

		if let Err(e) = result {
			insta::assert_snapshot!(e.to_string());
		} else {
			panic!("Expected an error, but parsing was successful");
		}
	}

	#[test]
	fn test_parse_element() {
		let input = "foo\nbar";
		let result = super::parse(input);

		assert!(result.is_ok());

		let bem_block = result.unwrap();

		assert_eq!(bem_block, super::BEMBlock {
			name: "foo".to_string(),
			modifiers: vec![],
			elements: vec![super::BEMElement {
				name: "bar".to_string(),
				modifiers: vec![],
			}],
		});
	}

	#[test]
	fn test_parse_element_with_dashes() {
		let input = "foo\nbar-baz-qux";
		let result = super::parse(input);

		assert!(result.is_ok());

		let bem_block = result.unwrap();

		assert_eq!(bem_block, super::BEMBlock {
			name: "foo".to_string(),
			modifiers: vec![],
			elements: vec![super::BEMElement {
				name: "bar-baz-qux".to_string(),
				modifiers: vec![],
			}],
		});
	}

	#[test]
	fn test_parse_element_with_modifier() {
		let input = "foo\nbar[baz]";
		let result = super::parse(input);

		assert!(result.is_ok());

		let bem_block = result.unwrap();

		assert_eq!(bem_block, super::BEMBlock {
			name: "foo".to_string(),
			modifiers: vec![],
			elements: vec![super::BEMElement {
				name: "bar".to_string(),
				modifiers: vec!["baz".to_string()],
			}],
		});
	}

	#[test]
	fn test_parse_element_with_modifiers() {
		let input = "foo\nbar[baz,qux]";
		let result = super::parse(input);

		assert!(result.is_ok());

		let bem_block = result.unwrap();

		assert_eq!(bem_block, super::BEMBlock {
			name: "foo".to_string(),
			modifiers: vec![],
			elements: vec![super::BEMElement {
				name: "bar".to_string(),
				modifiers: vec!["baz".to_string(), "qux".to_string()],
			}],
		});
	}

	#[test]
	fn test_parse_element_with_modifiers_and_spaces() {
		let input = "foo\nbar[  baz  ,  qux  ]";
		let result = super::parse(input);

		assert!(result.is_ok());

		let bem_block = result.unwrap();

		assert_eq!(bem_block, super::BEMBlock {
			name: "foo".to_string(),
			modifiers: vec![],
			elements: vec![super::BEMElement {
				name: "bar".to_string(),
				modifiers: vec!["baz".to_string(), "qux".to_string()],
			}],
		});
	}

	#[test]
	fn test_parse_element_with_modifiers_and_newlines_and_tabs() {
		let input = "foo\nbar[\n\tbaz,\n\tqux]";
		let result = super::parse(input);

		assert!(result.is_ok());

		let bem_block = result.unwrap();

		assert_eq!(bem_block, super::BEMBlock {
			name: "foo".to_string(),
			modifiers: vec![],
			elements: vec![super::BEMElement {
				name: "bar".to_string(),
				modifiers: vec!["baz".to_string(), "qux".to_string()],
			}],
		});
	}

	#[test]
	fn test_parse_element_with_modifiers_and_trailing_commas() {
		let input = "foo\nbar[baz,qux,]";
		let result = super::parse(input);

		assert!(result.is_ok());

		let bem_block = result.unwrap();

		assert_eq!(bem_block, super::BEMBlock {
			name: "foo".to_string(),
			modifiers: vec![],
			elements: vec![super::BEMElement {
				name: "bar".to_string(),
				modifiers: vec!["baz".to_string(), "qux".to_string()],
			}],
		});
	}

	#[test]
	fn test_parse_element_with_parentheses() {
		let input = "foo\nbar(baz,qux)";
		let result = super::parse(input);

		assert!(result.is_err());

		if let Err(e) = result {
			insta::assert_snapshot!(e.to_string());
		} else {
			panic!("Expected an error, but parsing was successful");
		}
	}

	#[test]
	fn test_parse_elements() {
		let input = "foo\nbar\nbaz\nqux";
		let result = super::parse(input);

		assert!(result.is_ok());

		let bem_block = result.unwrap();

		assert_eq!(bem_block, super::BEMBlock {
			name: "foo".to_string(),
			modifiers: vec![],
			elements: vec![
				super::BEMElement {
					name: "bar".to_string(),
					modifiers: vec![],
				},
				super::BEMElement {
					name: "baz".to_string(),
					modifiers: vec![],
				},
				super::BEMElement {
					name: "qux".to_string(),
					modifiers: vec![],
				}
			],
		});
	}

	#[test]
	fn test_parse_block_modifiers_and_element_modifiers() {
		let input = "a[b,c]\nd[e,f]\ng\nh[i]";
		let result = super::parse(input);

		assert!(result.is_ok());

		let bem_block = result.unwrap();

		assert_eq!(bem_block, super::BEMBlock {
			name: "a".to_string(),
			modifiers: vec!["b".to_string(), "c".to_string()],
			elements: vec![
				super::BEMElement {
					name: "d".to_string(),
					modifiers: vec!["e".to_string(), "f".to_string()],
				},
				super::BEMElement {
					name: "g".to_string(),
					modifiers: vec![],
				},
				super::BEMElement {
					name: "h".to_string(),
					modifiers: vec!["i".to_string()],
				}
			],
		});
	}

	#[test]
	fn test_parse_final_newlines() {
		let input = "foo\n\n\n";
		let result = super::parse(input);

		assert!(result.is_ok());

		let bem_block = result.unwrap();

		assert_eq!(bem_block, super::BEMBlock {
			name: "foo".to_string(),
			modifiers: vec![],
			elements: vec![],
		});
	}
}
