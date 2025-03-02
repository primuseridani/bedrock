// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::app::GraphicsContext;

use std::f64;
use std::iter;
use std::time::{SystemTime, UNIX_EPOCH};
use wgpu::{
	Color,
	LoadOp,
	Operations,
	RenderPassColorAttachment,
	RenderPassDescriptor,
	StoreOp,
};

impl GraphicsContext {
	pub fn render(&mut self) {
		let output = match self.surface.get_current_texture() {
			Ok(output) => output,

			Err(e) => panic!("unable to get current texture: {e}"),
		};

		let view = output.texture.create_view(&Default::default());

		let mut encoder = self.device.create_command_encoder(&Default::default());

		let colour = {
			let time = SystemTime::now()
				.duration_since(UNIX_EPOCH)
				.unwrap()
				.as_secs_f64();

			let hue = time / 4.0;

			hsva(hue, 1.0, 1.0, 1.0)
		};

		let _ = encoder.begin_render_pass(&RenderPassDescriptor {
			color_attachments: &[
				Some(RenderPassColorAttachment {
					view:           &view,
					resolve_target: None,

					ops: Operations {
						load: LoadOp::Clear(colour),

						store: StoreOp::Store,
					},
				}),
			],

			..Default::default()
		});

		self.queue.submit(iter::once(encoder.finish()));

		output.present();
	}
}

fn hsva(hue: f64, saturation: f64, value: f64, alpha: f64) -> Color {
	if saturation <= 0.0 {
		let value = value.clamp(0.0, 1.0);

		Color { r: value, g: value, b: value, a: alpha }
	} else {
		let h = hue % 1.0 * 6.0;
		let s = saturation.clamp(0.0, 1.0);
		let v = value.clamp(0.0, 1.0);
		let a = alpha;

		let f = h % 1.0;
		let p = v * (1.0 - s);
		let q = v * (-s).mul_add(f, 1.0); // v * (1.0 - s * f)
		let t = v * (-s).mul_add(1.0 - f, 1.0); // v * (1.0 - s * (1.0 - f))

		match h.trunc() as u8 {
			0x0 => Color { r: v, g: t, b: p, a },
			0x1 => Color { r: q, g: v, b: p, a },
			0x2 => Color { r: p, g: v, b: t, a },
			0x3 => Color { r: p, g: q, b: v, a },
			0x4 => Color { r: t, g: p, b: v, a },
			0x5 => Color { r: v, g: p, b: q, a },

			_ => unreachable!(),
		}
	}
}
