// Copyright 2025 Gabriel Bjørnager Jensen.

enable f16;

struct VertexInput {
	@location(0x0)
	global:  vec2<f16>,

	@location(0x1)
	texture: vec2<f16>,
};

struct VertexOutput {
	@builtin(position)
	clip: vec4<f32>,

	@location(0x0)
	texture: vec2<f32>,
};
