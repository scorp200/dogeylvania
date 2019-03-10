pub mod generator {

	extern crate rand;

	use rand::thread_rng;
	use rand::seq::SliceRandom;
	use rand::Rng;
	use crate::maps::{Map};
	use crate::tiles::{Tile};

	pub fn generate(map: &mut Map) {

		// Clear map to base.
		rectangle(map, 0, 0, map.width(), map.height(), Tile::tree);

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
			//rectangle(map, x-1, y-1, x+1, y+1, Tile::empty);

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

		for x in 2..width-1 {
			for y in 2..height-1 {
				if _grid[x][y] > 0 {
					let chance = [(false, 20), (true, 1)];
					let mut rng = thread_rng();
					if chance.choose_weighted(&mut rng, |item| item.1).unwrap().0 {
						rectangle(map, x-1, y-1, x+1, y+1, Tile::flower);
					} else {
						rectangle(map, x-1, y-1, x+1, y+1, Tile::empty);
					}
				}
				if _grid[x][y] > 6 {
					rectangle(map, x-1, y-1, x+1, y+1, Tile::tree);
				}
			}
		}

		createRoom(map, 20, 10, 10, 20);

	}

	fn createRoom(map: &mut Map, x: usize, y: usize, w: usize, h: usize) {
		rectangle(map, x, y, x+w, y+h, Tile::wall);
		rectangle(map, x+1, y+1, x+w-1, y+h-1, Tile::floor);

		// Left.
		if (!map.get(x-1, y+h/2).block_move) {
			rectangle(map, x, y+h/2, x+1, y+h/2+1, Tile::floor);
		}

		// Right.
		if (!map.get(x+w, y+h/2).block_move) {
			rectangle(map, x+w-1, y+h/2, x+w, y+h/2+1, Tile::floor);
		}

		// Top.
		if (!map.get(x+w/2, y-1).block_move) {
			rectangle(map, x+w/2, y, x+w/2+1, y+1, Tile::floor);
		}

		// Bottom.
		if (!map.get(x+w/2, y+h).block_move) {
			rectangle(map, x+w/2, y+h-1, x+w/2+1, y+h, Tile::floor);
		}
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
