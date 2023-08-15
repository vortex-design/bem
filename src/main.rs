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

fn run_cli(input_file: Option<String>, out: Option<String>) -> io::Result<String> {
	let mut bem_input = String::new();
	if let Some(input_file) = input_file.as_deref() {
		File::open(input_file)?.read_to_string(&mut bem_input)?;
	} else {
		io::stdin().read_to_string(&mut bem_input)?;
	}

	let bem_block = parse(&bem_input).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
	let json_output = bem::to_json(&bem_block)?;

	if let Some(out) = out.as_deref() {
		File::create(out)?.write_all(json_output.as_bytes())?;
	} else {
		io::stdout().write_all(json_output.as_bytes())?;
	}

	Ok(json_output)
}

fn main() {
	let cli = Cli::parse();
	if let Err(e) = run_cli(cli.input_file, cli.out) {
		eprintln!("An error occurred: {}", e);
		std::process::exit(1);
	}
}

#[cfg(test)]
mod tests {
	use super::run_cli;
	use tempfile::NamedTempFile;

	#[test]
	fn test_run_cli_with_valid_input() {
		// Set up some example BEM input as a string
		let bem_input = "media-player[dark]\nbutton[fast-forward,rewind]\ntimeline";

		// Write the BEM input to a temporary file
		let temp_input_file = NamedTempFile::new().unwrap();
		std::fs::write(temp_input_file.path(), bem_input).unwrap();

		// Create a temporary file for the output
		let temp_output_file = NamedTempFile::new().unwrap();

		// Run the CLI with the input and output files
		let result = run_cli(
			Some(temp_input_file.path().to_str().unwrap().to_string()),
			Some(temp_output_file.path().to_str().unwrap().to_string())
		);

		// Check that the result is Ok and the contents of the output file are correct
		assert!(result.is_ok());
		let output_content = std::fs::read_to_string(temp_output_file.path()).unwrap();

		insta::assert_snapshot!(output_content);
	}

	#[test]
	fn test_run_cli_with_invalid_input() {
		// Set up invalid BEM input as a string
		let invalid_bem_input = "media-player(dark)"; // <-- cannot use parentheses!

		// Write the invalid BEM input to a temporary file
		let temp_input_file = tempfile::NamedTempFile::new().unwrap();
		std::fs::write(temp_input_file.path(), invalid_bem_input).unwrap();

		// Run the CLI with the input file
		let result = run_cli(Some(temp_input_file.path().to_str().unwrap().to_string()), None);

		// Check that the result is an error, and that the error kind is what we expect
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidInput);
	}

	#[test]
	fn test_run_cli_with_nonexistent_input_file() {
		// Provide a nonexistent file path as the input file
		let nonexistent_input_file = "/path/to/nonexistent/file";

		// Run the CLI with the nonexistent input file
		let result = run_cli(Some(nonexistent_input_file.to_string()), None);

		// Check that the result is an error, and that the error kind is what we expect
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
	}
}
