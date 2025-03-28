// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::level::{Chunk, Layer, Level, Material};

use polywave::www::Html;
use std::borrow::Cow;

impl Level {
	pub fn load_builtin(name: &str) -> Option<Self> {
		match name {
			"field"     => Some(Self::FIELD),
			"mountain"  => Some(Self::MOUNTAIN),
			"valley"    => Some(Self::VALLEY),
			"lake"      => Some(Self::LAKE),
			"lava_lake" => Some(Self::LAVA_LAKE),

			_ => None,
		}
	}

	pub const FIELD: Self = Self {
		name:        Cow::Borrowed("Field"),
		creatour:    Cow::Borrowed("Achernar"),
		description: Cow::Borrowed("A flat field."),

		background: Html::from_u32(0x9DD8FEFF),

		chunks: Cow::Borrowed(&[
			Chunk {
				width: 1.0,

				layers: Cow::Borrowed(&[
					Layer {
						height: 1.0 / 3.0,

						material: Material::Dirt,
					},
				]),
			},
		]),
	};

	pub const MOUNTAIN: Self = Self {
		name:        Cow::Borrowed("Mountain"),
		creatour:    Cow::Borrowed("Achernar"),
		description: Cow::Borrowed("A simple mountain."),

		background: Html::from_u32(0xD0D0D0FF),

		chunks: Cow::Borrowed(&[
			Chunk {
				width: 0.5,

				layers: Cow::Borrowed(&[
					Layer {
						height: 1.0 / 3.0,

						material: Material::Rock,
					},
				]),
			},

			Chunk {
				width: 0.5,

				layers: Cow::Borrowed(&[
					Layer {
						height: 0.5,

						material: Material::Rock,
					},
				]),
			},
		]),
	};

	pub const VALLEY: Self = Self {
		name:        Cow::Borrowed("Valley"),
		creatour:    Cow::Borrowed("Achernar"),
		description: Cow::Borrowed("A simple valley."),

		background: Html::from_u32(0x017DA9FF),

		chunks: Cow::Borrowed(&[
			Chunk {
				width: 0.25,

				layers: Cow::Borrowed(&[
					Layer {
						height: 0.5,

						material: Material::Rock,
					},
				]),
			},

			Chunk {
				width: 0.5,

				layers: Cow::Borrowed(&[
					Layer {
						height: 0.125,

						material: Material::Rock,
					},

					Layer {
						height: 0.125,

						material: Material::Dirt,
					},
				]),
			},

			Chunk {
				width: 0.25,

				layers: Cow::Borrowed(&[
					Layer {
						height: 0.5,

						material: Material::Rock,
					},
				]),
			},
		]),
	};

	pub const LAKE: Self = Self {
		name:        Cow::Borrowed("Lake"),
		creatour:    Cow::Borrowed("Achernar"),
		description: Cow::Borrowed("A nice lake."),

		background: Html::from_u32(0xDB5F02FF),

		chunks: Cow::Borrowed(&[
			Chunk {
				width: 1.0 / 3.0,

				layers: Cow::Borrowed(&[
					Layer {
						height: 0.25,

						material: Material::Dirt,
					},
				]),
			},

			Chunk {
				width: 1.0 / 3.0,

				layers: Cow::Borrowed(&[
					Layer {
						height: 0.062500,

						material: Material::Dirt,
					},

					Layer {
						height: 0.125,

						material: Material::Water,
					},
				]),
			},

			Chunk {
				width: 1.0 / 3.0,

				layers: Cow::Borrowed(&[
					Layer {
						height: 0.25,

						material: Material::Dirt,
					},
				]),
			},
		]),
	};

	pub const LAVA_LAKE: Self = Self {
		name:        Cow::Borrowed("Lava Lake"),
		creatour:    Cow::Borrowed("Achernar"),
		description: Cow::Borrowed("A not-so-nice lake."),

		background: Html::from_u32(0xBA0628FF),

		chunks: Cow::Borrowed(&[
			Chunk {
				width: 1.0 / 3.0,

				layers: Cow::Borrowed(&[
					Layer {
						height: 0.25,

						material: Material::Dirt,
					},
				]),
			},

			Chunk {
				width: 1.0 / 3.0,

				layers: Cow::Borrowed(&[
					Layer {
						height: 0.062500,

						material: Material::Dirt,
					},

					Layer {
						height: 0.125,

						material: Material::Magma,
					},
				]),
			},

			Chunk {
				width: 1.0 / 3.0,

				layers: Cow::Borrowed(&[
					Layer {
						height: 0.25,

						material: Material::Dirt,
					},
				]),
			},
		]),
	};
}
