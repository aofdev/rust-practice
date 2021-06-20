use std::fs::File;
use thiserror::Error;

fn main() {
	let result_anyhow = use_anyhow();
	println!("{:?}", result_anyhow);

	let result_thiserror = use_thiserror();
	println!("{:?}", result_thiserror);
}

// Using is anyhow
fn use_anyhow() -> anyhow::Result<()> {
	let path = "hello.txt";
	let _content = anyhow::Context::with_context(std::fs::read(path), || {
		format!("Failed to read file {}", path)
	})?;

	Ok(())
}

// Using is thiserror
#[derive(Error, Debug)]
enum FileError {
	#[error("The format of your .txt file is invalid.")]
	InvalidFormat,
	// This wraps all IO errors produced by the std lib into our defined IOError
	#[error(transparent)]
	IOError(#[from] std::io::Error),
}

fn raise_format() -> Result<(), FileError> {
	return Err(FileError::InvalidFormat);
}

fn open_file() -> Result<(), FileError> {
	File::open("hello.txt")?;
	Ok(())
}

fn use_thiserror() -> Result<(), FileError> {
	match open_file() {
		Ok(_) => println!("Open file success"),
		Err(e) => println!("{}", e),
	}

	match raise_format() {
		Ok(_) => println!("Format success"),
		Err(e) => println!("{}", e),
	}

	Ok(())
}
