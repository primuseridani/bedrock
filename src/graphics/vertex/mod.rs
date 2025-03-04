// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::graphics::Vec4;

use std::mem::offset_of;
use wgpu::{
	BufferAddress,
	VertexAttribute,
	VertexBufferLayout,
	VertexFormat,
	VertexStepMode,
};
use zerocopy::{FromZeros, Immutable, IntoBytes, KnownLayout};

#[repr(C)]
#[derive(Clone, Debug, FromZeros, Immutable, IntoBytes, KnownLayout, PartialEq)]
pub(super) struct Vertex {
	pub position: Vec4,
	pub colour:   Vec4,
}

impl Vertex {
	pub const LAYOUT: VertexBufferLayout<'_> = VertexBufferLayout {
		array_stride: size_of::<Self>() as BufferAddress,
		step_mode:    VertexStepMode::Vertex,

		attributes: &[
			VertexAttribute {
				offset:          offset_of!(Self, position) as BufferAddress,
				shader_location: 0x0,
				format:          VertexFormat::Float32x4,
			},

			VertexAttribute {
				offset:          offset_of!(Self, colour) as BufferAddress,
				shader_location: 0x1,
				format:          VertexFormat::Float32x4,
			},
		],
	};
}

const _: () = assert!(Vertex::LAYOUT.attributes[0x0].offset == 0x00);
const _: () = assert!(Vertex::LAYOUT.attributes[0x1].offset == 0x10);
