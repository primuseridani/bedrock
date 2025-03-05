// Copyright 2025 Gabriel Bjørnager Jensen.

@binding(0x0)
@group(0x0)
var texture: texture_2d<f32>;

@binding(0x1)
@group(0x0)
var texture_sampler: sampler;

@fragment
//@must_use
fn fragment_main(
	input: VertexOutput,
) -> @location(0x0) vec4<f32> {
	return textureSample(texture, texture_sampler, input.texture);
}
