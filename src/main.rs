// Copyright 2025 Gabriel Bjørnager Jensen.

#![feature(cold_path)]
#![feature(default_field_values)]
#![feature(generic_arg_infer)]

// Why is this needed?
#![expect(clippy::module_inception)]

extern crate self as bedrock;

const _: () = assert!(usize::BITS >= u32::BITS);

mod app;
mod config;
mod error;
mod graphics;
mod level;
mod map;
mod log;
mod version;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL_ALLOCATOR: Jemalloc = Jemalloc;

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
