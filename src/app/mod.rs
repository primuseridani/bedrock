// Copyright 2025 Gabriel Bjørnager Jensen.

mod app;
mod config;
mod user_event;

pub use app::App;

use config::Config;
use user_event::UserEvent;

pub const DEFAULT_PORT: u16 = 0x4274;
