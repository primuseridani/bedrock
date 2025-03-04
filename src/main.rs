// Copyright 2025 Gabriel Bjørnager Jensen.

#![feature(thread_sleep_until)]

// Why is this needed?
#![allow(clippy::module_inception)]

const _: () = assert!(usize::BITS >= u32::BITS);

mod app;
mod config;
mod error;
mod graphics;
mod level;
mod version;

use crate::app::App;
use crate::error::Result;

use std::process::exit;

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
