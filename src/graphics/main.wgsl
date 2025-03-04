// Copyright 2025 Gabriel Bjørnager Jensen.

struct VertexInput {
	@location(0x0) position: vec4<f32>,
	@location(0x1) colour:   vec4<f32>,
};

struct VertexOutput {
	@builtin(position) clip:   vec4<f32>,
	@location(0x0)     colour: vec4<f32>,
};

@vertex
//@must_use
fn vertex_main(
	input: VertexInput,
) -> VertexOutput {
	var out: VertexOutput;

	out.clip   = input.position;
	out.colour = input.colour;

	return out;
}

@fragment
//@must_use
fn fragment_main(
	input: VertexOutput,
) -> @location(0x0) vec4<f32> {
	return input.colour;
}
