// Copyright 2025 Gabriel Bjørnager Jensen.

#![feature(thread_sleep_until)]

mod app;
mod error;
mod level;

use crate::app::App;
use crate::error::Result;

use std::process::exit;

const _: () = assert!(usize::BITS >= u32::BITS);

fn main() -> ! {
	let run = || -> Result<()> {
		let app = App::new()?;
		app.run()?;

		Ok(())
	};

	let code = if let Err(e) = run() {
		eprintln!("error: {e}");

		e.into()
	} else {
		0x0
	};

	exit(code);
}
