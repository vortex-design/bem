use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BEMBlock {
	pub name: String,
	pub modifiers: Vec<String>,
	pub elements: Vec<BEMElement>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BEMElement {
	pub name: String,
	pub modifiers: Vec<String>,
}
