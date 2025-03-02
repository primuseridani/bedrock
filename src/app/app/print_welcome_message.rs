// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::app::App;
use crate::version::Version;

impl App {
	pub(super) fn print_welcome_message(&self) {
		eprintln!();
		eprintln!("YOU HAVE NOW HIT BEDROCK!");
		eprintln!("bedrock-{}", Version::CURRENT);
		eprintln!();
		eprintln!("Copyright \u{00A9} 2025 Gabriel Bj\u{00F8}rnager Jensen.");
		eprintln!();
	}
}
