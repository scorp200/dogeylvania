pub mod maps {
	use crate::dogemaths::*;
	use crate::tiles::*;
	use tcod::colors::{self, Color};
	pub struct Map {
		_width: usize,
		_height: usize,
		_map: Vec<Vec<Tile>>,
	}

	impl Map {
		pub fn new(width: usize, height: usize, map: Vec<Vec<Tile>>) -> Self {
			Map {
				_width: width,
				_height: height,
				_map: map,
			}
		}

		pub fn new_default(width: usize, height: usize, default: Tile) -> Self {
			let map = vec![vec![default; height]; width];
			Map::new(width, height, map)
		}

		pub fn is_blocked(map: &Map, x: usize, y: usize) -> bool {
			!map._map[x][y].block_move
		}

		pub fn width(&self) -> usize {
			self._width
		}

		pub fn height(&self) -> usize {
			self._height
		}

		pub fn get(&mut self, x: usize, y: usize) -> &mut Tile {
			&mut self._map[clamp(x, 0, self._width - 1)][clamp(y, 0, self._height)]
		}

		pub fn set(&mut self, x: usize, y: usize, value: Tile) {
			self._map[clamp(x, 0, self._width - 1)][clamp(y, 0, self._height)] = value;
		}
	}
}
