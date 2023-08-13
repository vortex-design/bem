use models::BEMBlock;

pub use parser::parse_bem;

mod models;
mod parser;

pub fn convert_to_json(bem_block: &BEMBlock) -> Result<String, serde_json::Error> {
	// Convert the BEM block into JSON
	let json_output = serde_json::to_string(&bem_block)?;

	Ok(json_output)
}
