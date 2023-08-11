use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct BEMBlock {
	pub name: String,
	pub modifiers: Vec<String>,
	pub elements: Vec<BEMElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BEMElement {
	pub name: String,
	pub modifiers: Vec<String>,
}
