#![allow(non_upper_case_globals)]

use std::{env, fs, path::Path, io, process};

mod omctextdecoder;

fn main() {
	let Some(path) = env::args().nth(1) else {
		let program = env::current_exe().unwrap();
		let program = program.file_name().unwrap();
		println!("Usage: {} <path>", program.to_string_lossy());
		process::exit(1);
	};

	let did_decode = decode_dir(path.as_ref()).unwrap_or_else(|err| {
		eprintln!("{err}");
		process::exit(1);
	});

	if !did_decode {
		println!("No files decoded");
	}
}

fn decode_dir(path: &Path) -> io::Result<bool> {
	if path.is_dir() {
		fs::read_dir(path)?
			.try_fold(false, |prev, entry| {
				Ok(decode_dir(&entry?.path())? || prev)
			})
	} else {
		omctextdecoder::decode(path)
	}
}
