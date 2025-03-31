// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::graphics::Vec2;

use std::mem::offset_of;
use zerocopy::{FromZeros, Immutable, IntoBytes, KnownLayout};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, FromZeros, Immutable, IntoBytes, KnownLayout, PartialEq)]
pub(super) struct Vertex {
	pub clip:    Vec2,
	pub texture: Vec2,
}

impl Vertex {
	pub const LAYOUT: wgpu::VertexBufferLayout<'_> = wgpu::VertexBufferLayout {
		array_stride: size_of::<Self>() as wgpu::BufferAddress,
		step_mode:    wgpu::VertexStepMode::Vertex,

		attributes: &[
			wgpu::VertexAttribute {
				offset:          offset_of!(Self, clip) as wgpu::BufferAddress,
				shader_location: 0x0,
				format:          wgpu::VertexFormat::Float16x4,
			},

			wgpu::VertexAttribute {
				offset:          offset_of!(Self, texture) as wgpu::BufferAddress,
				shader_location: 0x1,
				format:          wgpu::VertexFormat::Float16x2,
			},
		],
	};
}

const _: () = assert!(Vertex::LAYOUT.attributes[0x0].offset == 0x0);
const _: () = assert!(Vertex::LAYOUT.attributes[0x1].offset == 0x4);
