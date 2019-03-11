pub mod maps {
	use crate::dogemaths::*;
	use crate::tiles::*;
	//use tcod::colors::{self, Color};
	pub struct Map {
		_width: usize,
		_height: usize,
		_map: Vec<Vec<Tile>>,
		_floor: i32,
	}

	impl Map {
		pub fn new(width: usize, height: usize) -> Self {
			let mut map = Vec::new();
			for _x in 0..width {
				let mut col = Vec::new();
				for _y in 0..height {
					col.push(Tile::empty());
				}
				map.push(col);
			}

			Map {
				_width: width,
				_height: height,
				_map: map,
				_floor: 1,
			}
		}

		pub fn re_default(&mut self) {
			for x in 0..self._width {
				for y in 0..self._height {
					self.set(x, y, Tile::empty());
				}
			}
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

		pub fn set_floor(&mut self, value: i32) {
			self._floor = value;
		}

		pub fn get_floor(&self) -> i32 {
			self._floor
		}
	}
}
