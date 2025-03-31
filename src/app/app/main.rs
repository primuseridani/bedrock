// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::app::App;
use crate::log::log;

impl App {
	pub fn main() -> ! {
		use std::process::exit;

		let code = if let Err(e) = Self::run() {
			log!(error, "{e}");

			e.into()
		} else {
			0x0
		};

		exit(code);
	}
}
