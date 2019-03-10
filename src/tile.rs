pub mod tiles {

	extern crate rand;

	use tcod::colors::{self, Color};
	use rand::thread_rng;
	use rand::seq::SliceRandom;

	#[derive(PartialEq, Debug)]
	pub struct Tile {
		pub char: Option<(char, Color, Color)>,
		pub color: Option<(Color, Color)>,
		pub block_move: bool,
		pub block_light: bool,
		pub explored: bool,
	}

	impl Tile {
		pub fn empty() -> Self {
			let mut rng = thread_rng();
			let c = [',', '.', ' '];
			Tile {
				char: Some((
					*c.choose(&mut rng).unwrap(),
					Color { r: 66, g: 46, b: 25 },
					Color { r: 140, g: 99, b: 56 }
				)),
				color: Some((
					Color { r: 0, g: 0, b: 0 },
					Color { r: 40, g: 40, b: 10 }
				)),
				block_move: false,
				block_light: false,
				explored: false,
			}
		}

		pub fn tree() -> Self {
			let mut rng = thread_rng();
			let c = [5 as char, 6 as char, 24 as char, 30 as char];
			Tile {
				char: Some((
					*c.choose(&mut rng).unwrap(),
					Color { r: 0, g: 20, b: 0 },
					Color { r: 0, g: 100, b: 0 }
				)),
				color: Some((
					Color { r: 0, g: 0, b: 0 },
					Color { r: 20, g: 20, b: 5 }
				)),
				block_move: true,
				block_light: true,
				explored: false,
			}
		}

		pub fn wall() -> Self {
			let mut rng = thread_rng();
			let c = ['#'];
			Tile {
				char: Some((
					*c.choose(&mut rng).unwrap(),
					Color { r: 100, g: 46, b: 25 },
					Color { r: 250, g: 99, b: 56 }
				)),
				color: Some((
					Color { r: 0, g: 0, b: 0 },
					Color { r: 20, g: 20, b: 5 }
				)),
				block_move: true,
				block_light: true,
				explored: false,
			}
		}

		pub fn floor() -> Self {
			let mut rng = thread_rng();
			let c = [178 as char];
			Tile {
				char: Some((
					*c.choose(&mut rng).unwrap(),
					Color { r: 66, g: 46, b: 25 },
					Color { r: 140, g: 99, b: 56 }
				)),
				color: Some((
					Color { r: 50, g: 46, b: 25 },
					Color { r: 100, g: 99, b: 56 }
				)),
				block_move: false,
				block_light: false,
				explored: false,
			}
		}

		pub fn flower() -> Self {
			let mut rng = thread_rng();
			let colors = [
				(Color { r: 60, g: 60, b: 0 },
				Color { r: 255, g: 255, b: 0 }),
				(Color { r: 60, g: 0, b: 60 },
				Color { r: 255, g: 0, b: 255 })
			];
			let c = *colors.choose(&mut rng).unwrap();
			Tile {
				char: Some((
					'+',
					c.0,
					c.1
				)),
				color: Some((
					Color { r: 0, g: 0, b: 0 },
					Color { r: 40, g: 40, b: 10 }
				)),
				block_move: false,
				block_light: false,
				explored: false,
			}
		}
	}
}
