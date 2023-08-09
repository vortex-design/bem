#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar/bem.pest"]
pub struct BEMParser;

fn main() {
	let bem_content = "media-player(dark|light)\nbutton(fast-forward|rewind)\ntimeline";
	let parsed = BEMParser::parse(Rule::bem, bem_content).expect("Failed to parse BEM content");

	// Process the parsed content...
	for pair in parsed {
		println!("{:?}", pair);
	}
}
