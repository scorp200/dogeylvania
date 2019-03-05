pub mod maps {
	use crate::dogemaths::*;
	use tcod::colors::{self, Color};
	pub struct Map {
		_width: usize,
		_height: usize,
		_map: Vec<Vec<usize>>,
	}

	const MAP_INDEX: &'static [(char, Color)] = &[
		(',', colors::DARK_GREEN),
		('^', colors::DARK_GREY),
		('^', colors::DARK_GREY),
		('^', colors::DARK_GREY),
		('^', colors::DARK_GREY),
		('^', colors::DARK_GREY),
		('^', colors::DARK_GREY),
	];

	impl Map {
		pub fn new(width: usize, height: usize, map: Vec<Vec<usize>>) -> Self {
			Map {
				_width: width,
				_height: height,
				_map: map,
			}
		}

		pub fn new_default(width: usize, height: usize, default: usize) -> Self {
			let map = vec![vec![default; height]; width];
			Map::new(width, height, map)
		}

		pub fn width(&self) -> usize {
			self._width
		}

		pub fn height(&self) -> usize {
			self._height
		}

		pub fn get(&self, x: usize, y: usize) -> usize {
			self._map[clamp(x, 0, self._width - 1)][clamp(y, 0, self._height)]
		}

		pub fn get_char(&self, x: usize, y: usize) -> char {
			MAP_INDEX[self._map[x][y]].0
		}
		pub fn get_color(&self, x: usize, y: usize) -> Color {
			MAP_INDEX[self._map[x][y]].1
		}

		pub fn set(&mut self, x: usize, y: usize, value: usize) {
			self._map[clamp(x, 0, self._width - 1)][clamp(y, 0, self._height)] = value;
		}
	}
}
