// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::app::App;
use crate::version::Version;

impl App {
	pub(super) fn print_welcome_message() {
		eprintln!();
		eprintln!("\u{001B}[001mYOU HAVE NOW HIT BEDROCK!\u{001B}[022m");
		eprintln!("\u{001B}[002mbedrock-{}\u{001B}[022m", Version::CURRENT);
		eprintln!();
		eprintln!("\u{001B}[002mCopyright \u{00A9} 2025 Gabriel Bj\u{00F8}rnager Jensen.\u{001B}[022m");
		eprintln!();
		eprintln!("Controls (en-gb):");
		eprintln!("  esc                : pause / unpause");
		eprintln!("  q                  : quit (whilst paused)");
		eprintln!();
		eprintln!("  mwheelup           : pan right");
		eprintln!("  mwheeldown         : pan left");
		eprintln!("  mwheel2up OR");
		eprintln!("  shift + mwheelup   : pan up");
		eprintln!("  mwheel2down OR");
		eprintln!("  shift + mwheeldown : pan down");
		eprintln!("  ctrl + mwheelup    : zoom in");
		eprintln!("  ctrl + mwheeldown  : zoom out");
		eprintln!();
		eprintln!("  plus               : increas tps");
		eprintln!("  hyphen             : decrease tps");
		eprintln!();
	}
}
