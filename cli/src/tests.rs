use clap::{
	CommandFactory,
	Parser,
};

use super::*;

#[test]
fn clap_check() {
	Args::command().debug_assert();

	let tests: &[&[&str]] = &[
		&["template", "-o", "foo"],
		&["generate", "foo.mid", "-o", "bar.txt"],
		&[
			"remap",
			"--from-mapping=from.txt",
			"--to-mapping=to.txt",
			"bar.mid",
			"--out=lol.mid",
		],
	];

	for &args in tests {
		let mut cmd = vec!["drum-mapper"];
		cmd.extend_from_slice(args);
		if let Err(e) = Args::try_parse_from(&cmd) {
			panic!("\ntest with command {args:?} failed:\n{e}");
		}
	}
}
