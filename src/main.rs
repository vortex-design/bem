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

pub trait StringReader {
	fn read_to_string(&mut self, s: &mut String) -> io::Result<()>;
}

pub struct StdinReader;

impl StringReader for StdinReader {
	#[cfg(not(tarpaulin_include))]
	fn read_to_string(&mut self, s: &mut String) -> io::Result<()> {
		io::stdin()
			.read_to_string(s)
			.map(|_| ())
	}
}

trait ContentReader {
	fn read_content(&mut self) -> io::Result<String>;
}

struct FileOrStdinReader {
	input_file: Option<String>,
}

fn read_input_file_or_stdin<R: StringReader>(
	input_file: Option<&str>,
	bem_input: &mut String,
	reader: &mut R
) -> io::Result<()> {
	if let Some(input_file) = input_file {
		File::open(input_file)?.read_to_string(bem_input)?;
	} else {
		reader.read_to_string(bem_input)?;
	}
	Ok(())
}

impl ContentReader for FileOrStdinReader {
	fn read_content(&mut self) -> io::Result<String> {
		let mut bem_input = String::new();
		let mut reader = StdinReader;
		read_input_file_or_stdin(self.input_file.as_deref(), &mut bem_input, &mut reader)?;
		Ok(bem_input)
	}
}

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

fn run_cli<R: ContentReader>(mut reader: R, out: Option<String>) -> io::Result<String> {
	let bem_input = reader.read_content()?;

	let bem_block = parse(&bem_input).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
	let json_output = bem::to_json(&bem_block)?;

	if let Some(out) = out.as_deref() {
		File::create(out)?.write_all(json_output.as_bytes())?;
	} else {
		io::stdout().write_all(json_output.as_bytes())?;
	}

	Ok(json_output)
}

#[cfg(not(tarpaulin_include))]
fn main() {
	let cli = Cli::parse();
	let reader = FileOrStdinReader { input_file: cli.input_file };
	if let Err(e) = run_cli(reader, cli.out) {
		eprintln!("An error occurred: {}", e);
		std::process::exit(1);
	}
}

#[cfg(test)]
mod tests {
	use super::{ run_cli, FileOrStdinReader };
	use tempfile::NamedTempFile;

	pub struct MockInputReader {
		content: String,
	}

	impl super::StringReader for MockInputReader {
		fn read_to_string(&mut self, s: &mut String) -> std::io::Result<()> {
			s.push_str(&self.content);
			Ok(())
		}
	}

	struct MockStdinReader {
		content: String,
	}

	impl super::ContentReader for MockStdinReader {
		fn read_content(&mut self) -> std::io::Result<String> {
			Ok(self.content.clone())
		}
	}

	const VALID_CONTENT: &str = "media-player[dark]\nbutton[fast-forward,rewind]\ntimeline";
	const INVALID_CONTENT: &str = "media-player(dark)";

	#[test]
	fn test_read_input_file_or_stdin_with_stdin() {
		let content = "content from standard input".to_string();
		let mut mock_reader = MockInputReader { content };
		let mut bem_input = String::new();

		let result = super::read_input_file_or_stdin(None, &mut bem_input, &mut mock_reader);

		assert!(result.is_ok());
		assert_eq!(bem_input, "content from standard input");
	}

	#[test]
	fn test_run_cli_with_stdin() {
		let mock_reader = MockStdinReader {
			content: VALID_CONTENT.to_string(),
		};
		let temp_output_file = NamedTempFile::new().unwrap();
		let result = run_cli(
			mock_reader,
			Some(temp_output_file.path().to_str().unwrap().to_string())
		);

		assert!(result.is_ok());
		let output_content = std::fs::read_to_string(temp_output_file.path()).unwrap();
		insta::assert_snapshot!(output_content);
	}

	#[test]
	fn test_run_cli_with_stdin_content() {
		let mock_reader = MockStdinReader {
			content: VALID_CONTENT.to_string(),
		};
		let result = run_cli(mock_reader, None);

		assert!(result.is_ok());
		insta::assert_snapshot!(result.unwrap());
	}

	#[test]
	fn test_run_cli_with_valid_input() {
		// Set up some example BEM input as a string
		let bem_input = VALID_CONTENT;

		// Write the BEM input to a temporary file
		let temp_input_file = NamedTempFile::new().unwrap();
		std::fs::write(temp_input_file.path(), bem_input).unwrap();

		// Create a temporary file for the output
		let temp_output_file = NamedTempFile::new().unwrap();

		// Create a reader with the input file
		let reader = FileOrStdinReader {
			input_file: Some(temp_input_file.path().to_str().unwrap().to_string()),
		};

		// Run the CLI with the reader and output file
		let result = run_cli(reader, Some(temp_output_file.path().to_str().unwrap().to_string()));

		// Check that the result is Ok and the contents of the output file are correct
		assert!(result.is_ok());
		let output_content = std::fs::read_to_string(temp_output_file.path()).unwrap();

		insta::assert_snapshot!(output_content);
	}

	#[test]
	fn test_run_cli_with_invalid_input() {
		// Create a mock reader with the invalid BEM input as a string
		let mock_reader = MockStdinReader {
			content: INVALID_CONTENT.to_string(), // <-- cannot use parentheses!
		};

		// Run the CLI with the reader
		let result = run_cli(mock_reader, None);

		// Check that the result is an error, and that the error kind is what we expect
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidInput);
	}

	#[test]
	fn test_run_cli_with_nonexistent_input_file() {
		// Create a reader with a nonexistent file path as the input file
		let reader = FileOrStdinReader {
			input_file: Some("/path/to/nonexistent/file".to_string()),
		};

		// Run the CLI with the reader
		let result = run_cli(reader, None);

		// Check that the result is an error, and that the error kind is what we expect
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
	}
}
