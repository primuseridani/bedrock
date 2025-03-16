// Copyright 2025 Gabriel Bjørnager Jensen.

#![feature(generic_arg_infer)]
#![feature(thread_sleep_until)]

// Why is this needed?
#![allow(clippy::module_inception)]

const _: () = assert!(usize::BITS >= u32::BITS);

mod app;
mod error;
mod graphics;
mod level;
mod map;
mod log;
mod preset;
mod version;

fn main() -> ! {
	use crate::app::App;
	use crate::log::log;

	use std::process::exit;

	let code = if let Err(e) = App::run() {
		log!(error, "{e}");

		e.into()
	} else {
		0x0
	};

	exit(code);
}
