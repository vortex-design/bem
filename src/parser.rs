use pest::Parser;
use pest_derive::Parser;
use crate::models::{ BEMBlock, BEMElement };

#[derive(Parser)]
#[grammar = "grammar/bem.pest"]
pub struct BEMParser;

impl BEMParser {
	/// Parses a BEM string into a `BEMBlock` structure.
	///
	/// # Examples
	///
	/// ```
	/// use bem::parse_bem;
	///
	/// let input = "media-player(dark|light)\nbutton(fast-forward|rewind)\ntimeline";
	/// let result = parse_bem(input);
	///
	/// assert!(result.is_ok());
	/// ```
	///
	/// # Errors
	///
	/// This function will return an error if the input string is not a valid BEM string according to the grammar.
	pub fn parse_bem(input: &str) -> Result<BEMBlock, String> {
		let bem = BEMParser::parse(Rule::bem, input)
			.map_err(|e| format!("Parsing error: {}", e))?
			.next()
			.unwrap();

		let mut name = String::new();
		let mut modifiers = Vec::new();
		let mut elements = Vec::new();

		for pair in bem.into_inner() {
			match pair.as_rule() {
				Rule::block => {
					let (block_name, block_modifiers) = Self::parse_part(pair)?;
					name = block_name;
					modifiers = block_modifiers;
				}
				Rule::element => {
					let (element_name, element_modifiers) = Self::parse_part(pair)?;
					elements.push(BEMElement {
						name: element_name,
						modifiers: element_modifiers,
					});
				}
				_ => {
					return Err(format!("Unexpected rule: {:?}", pair.as_rule()));
				}
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
					modifiers = inner_pair
						.into_inner()
						.map(|p| p.as_str().to_string())
						.collect();
				}
				_ => {
					return Err(format!("Unexpected rule: {:?}", inner_pair.as_rule()));
				}
			}
		}

		Ok((name, modifiers))
	}
}

#[cfg(test)]
mod tests {
	use super::BEMParser;

	#[test]
	fn test_parse_valid_bem() {
		let input = "media-player(dark|light)\nbutton(fast-forward|rewind)\ntimeline";
		let result = BEMParser::parse_bem(input);

		assert!(result.is_ok());

		let _bem_block = result.unwrap();
		// Add more assertions here to check the content of bem_block
	}

	#[test]
	fn test_parse_invalid_bem() {
		let input = "invalid input";
		let result = BEMParser::parse_bem(input);

		assert!(result.is_err());
	}

	#[test]
	fn test_parse_bem() {
		let input = "block-name(mod-1|mod-2)\nelement-one(mod-3)\nelement-two";
		let result = BEMParser::parse_bem(input);

		assert!(result.is_ok());

		// let _bem_block = result.unwrap();

		// // Checking the block name
		// assert_eq!(bem_block.name, "block-name");

		// // Checking the block's modifiers
		// assert_eq!(bem_block.modifiers.len(), 2);
		// assert_eq!(bem_block.modifiers[0], "mod-1");
		// assert_eq!(bem_block.modifiers[1], "mod-2");

		// // Checking the elements
		// assert_eq!(bem_block.elements.len(), 2);

		// // Checking the first element
		// assert_eq!(bem_block.elements[0].name, "element-one");
		// assert_eq!(bem_block.elements[0].modifiers.len(), 1);
		// assert_eq!(bem_block.elements[0].modifiers[0], "mod-3");

		// // Checking the second element
		// assert_eq!(bem_block.elements[1].name, "element-two");
		// assert_eq!(bem_block.elements[1].modifiers.len(), 0);
	}
}
