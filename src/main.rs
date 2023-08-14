//! This tool provides a command-line interface for working with BEM (Block Element Modifier) notation.
//! It leverages the `bem` library to parse, serialize, and deserialize BEM structures, providing
//! powerful functionality in a convenient CLI format.
//!
//! # Usage
//!
//! Run the following command for a specific task:
//!
//! - `cat media-player.bem | bem`: Parse BEM notation from stdin.
//! - `echo [INPUT] | bem`: Parse BEM notation from piped input.
//!
//! # Examples
//!
//! ```
//! $ cat media-player.bem | bem
//! $ echo "media-player[dark]\nbutton[fast-forward,rewind]\ntimeline" | bem
//! ```
//!
//! Please refer to the individual command documentation for detailed information and options.

use clap::Parser;
use std::fs::File;
use std::io::{ self, Read, Write };
use bem::parse;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
	/// Input file name (default: <stdin>)
	#[arg(value_name = "INPUT_FILE")]
	input_file: Option<String>,

	/// Output file name (default: <stdout>)
	#[arg(short, long, value_name = "OUTPUT_FILE")]
	out: Option<String>,
}

fn main() -> io::Result<()> {
	let cli = Cli::parse();

	let mut bem_input = String::new();
	if let Some(input_file) = cli.input_file.as_deref() {
		File::open(input_file)?.read_to_string(&mut bem_input)?;
	} else {
		io::stdin().read_to_string(&mut bem_input)?;
	}

	let bem_block = parse(&bem_input).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
	let json_output = bem::to_json(&bem_block)?;

	if let Some(out) = cli.out.as_deref() {
		println!("Value for out: {}", out);
		File::create(out)?.write_all(json_output.as_bytes())?;
	} else {
		io::stdout().write_all(json_output.as_bytes())?;
	}

	Ok(())
}
