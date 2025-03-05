// Copyright 2025 Gabriel Bjørnager Jensen.

@vertex
//@must_use
fn vertex_main(
	input: VertexInput,
) -> VertexOutput {
	var out: VertexOutput;

	out.clip    = vec4(input.global, 0.0, 1.0);
	out.texture = input.texture;

	return out;
}
