pub mod generator {

	extern crate rand;

	use rand::Rng;
	use crate::maps::{Map};
	use crate::tiles::{Tile};

	pub fn generate(map: &mut Map) {

		// Create temporary grid.
		let width = map.width();
		let height = map.height();
		println!("{} {}", width, height);
		let mut _grid = vec![vec![0; height]; width];
		let mut x = width / 2;
		let mut y = height / 2;
		for _ in 0..height*width {

			_grid[x][y] += 1;
			//map.set(x, y, Tile::empty());
			rectangle(map, x-1, y-1, x+1, y+1, Tile::empty);

			x += randomOffset();
			y += randomOffset();
			x -= 1;
			y -= 1;

			if x < 2
			|| y < 2
			|| x > width-3
			|| y > height-3 {
				x = width / 2;
				y = height / 2;
			}
		}

		//rectangle(map, 1, 1, map.width()-1, map.height()-1, Tile::empty);

		// Room test.
		//rectangle(map, 10, 10, 30, 20, Tile::wall);		// Wall.
		//rectangle(map, 20, 10, 21, 11, Tile::empty);	// Door.
		//rectangle(map, 11, 11, 29, 19, Tile::empty);	// Interior.
		//rectangle(map, 15, 15, 18, 18, Tile::gold);		// Goooooold.
	}

	pub fn findOpenSpace(map: &mut Map) -> (usize, usize) {
		let mut rng = rand::thread_rng();
		let width = map.width();
		let height = map.height();
		let mut ox = 0;
		let mut oy = 0;
		for _ in 0..height*width {
			let x = rng.gen_range(0, width);
			let y = rng.gen_range(0, height);
			if !map.get(x, y).block_move {
				ox = x;
				oy = y;
			}
		}
		(ox, oy)
	}

	fn rectangle(map: &mut Map, x1: usize, y1: usize, x2: usize, y2: usize, t: fn() -> Tile) {
		for y in y1..y2 as usize {
			for x in x1..x2 as usize {
				map.set(x, y, t());
			}
		}
	}

	fn randomOffset() -> usize {
		let mut rng = rand::thread_rng();
		rng.gen_range(0, 3)
	}
}
