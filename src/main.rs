// Copyright 2025 Gabriel Bjørnager Jensen.

#![feature(thread_sleep_until)]

mod app;
mod level;

const _: () = assert!(usize::BITS >= u32::BITS);

fn main() {
	let config = Default::default();

	let app = crate::app::App::new(config);
	app.run();
}
