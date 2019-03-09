pub mod tiles {
	use tcod::colors::{self, Color};
	//#[derive(Clone, Copy, PartialEq, Debug)]
	pub struct Tile {
		pub char: Option<(char, Color, Color)>,
		pub color: Option<(Color, Color)>,
		pub block_move: bool,
		pub block_light: bool,
		pub explored: bool,
	}

	impl Tile {
		pub fn empty() -> Self {
			Tile {
				char: Some((
					'.',
					Color {
						r: 50,
						g: 50,
						b: 50,
					},
					Color {
						r: 255,
						g: 255,
						b: 255,
					}
				)),
				color: Some((
					Color {
						r: 0,
						g: 0,
						b: 0,
					},
					Color {
						r: 40,
						g: 40,
						b: 10,
					},
				)),
				block_move: false,
				block_light: false,
				explored: false,
			}
		}

		pub fn wall() -> Self {
			Tile {
				char: Some((
					'#',
					Color {
						r: 40,
						g: 10,
						b: 0,
					},
					Color {
						r: 200,
						g: 50,
						b: 0,
					}
				)),
				color: Some((
					Color {
						r: 0,
						g: 0,
						b: 0,
					},
					Color {
						r: 20,
						g: 20,
						b: 5,
					},
				)),
				block_move: true,
				block_light: true,
				explored: false,
			}
		}

		pub fn gold() -> Self {
			Tile {
				char: Some((
					'*',
					Color {
						r: 60,
						g: 60,
						b: 0,
					},
					Color {
						r: 255,
						g: 255,
						b: 0,
					}
				)),
				color: Some((
					Color {
						r: 0,
						g: 0,
						b: 0,
					},
					Color {
						r: 20,
						g: 20,
						b: 5,
					},
				)),
				block_move: true,
				block_light: true,
				explored: false,
			}
		}
	}
}
