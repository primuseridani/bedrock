// Copyright 2025 Gabriel Bjørnager Jensen.

#![allow(internal_features)]

#![feature(cold_path)]
#![feature(default_field_values)]
#![feature(generic_arg_infer)]
#![feature(rustc_attrs)]

extern crate self as bedrock;

const _: () = assert!(usize::BITS >= u32::BITS);

mod app;
mod config;
mod error;
mod graphics;
mod level;
mod map;
mod log;
mod player;
mod version;

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL_ALLOCATOR: Jemalloc = Jemalloc;

fn main() -> ! {
	app::App::main();
}
