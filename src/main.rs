#![allow(non_upper_case_globals)]

use std::io::Result;
use std::fs::read_dir;
use std::path::Path;

mod omctextdecoder;

fn main() {
	let from = std::env::args().nth(1).unwrap_or(".".into());
	if let Err(err) = walk_dir(from.as_ref()) {
		eprintln!("{err}");
	}
}

fn walk_dir(from: &Path) -> Result<()> {
	for entry in read_dir(from)? {
		let path = entry?.path();

		if path.is_dir() {
			walk_dir(&path)?;
		} else {
			omctextdecoder::decode(&path)?;
		}
	}
	Ok(())
}
