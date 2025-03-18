// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::graphics::InitGraphicsContext;

use std::hint::cold_path;
use winit::event_loop::ActiveEventLoop;

#[expect(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum GraphicsContext {
	Uninit,

	Init(InitGraphicsContext),
}

impl GraphicsContext {
	#[inline(always)]
	#[must_use]
	pub const fn new() -> Self {
		Self::Uninit
	}

	#[expect(unused)]
	#[inline(always)]
	pub fn init(&mut self, event_loop: &ActiveEventLoop) {
		Self::init_with(self, event_loop, |_| { });
	}

	#[inline]
	#[track_caller]
	pub fn init_with<F: FnOnce(&mut InitGraphicsContext)>(&mut self, event_loop: &ActiveEventLoop, op: F) {
		if self.is_init() {
			return;
		}

		// This should only happen once per run.
		cold_path();

		let mut context = InitGraphicsContext::new(event_loop);

		op(&mut context);

		*self = Self::Init(context);
	}

	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub const fn unwrap(&mut self) -> &mut InitGraphicsContext {
		let Self::Init(ref mut context) = *self else {
			panic!("expected initialised graphics context but found none");
		};

		context
	}

	#[expect(unused)]
	#[inline(always)]
	#[must_use]
	pub const fn is_uninit(&self) -> bool {
		matches!(*self, Self::Uninit)
	}

	#[inline(always)]
	#[must_use]
	pub const fn is_init(&self) -> bool {
		matches!(*self, Self::Init(_))
	}
}

impl Default for GraphicsContext {
	#[inline(always)]
	fn default() -> Self {
		Self::new()
	}
}
