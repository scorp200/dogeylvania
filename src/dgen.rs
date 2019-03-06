pub mod generator {
	use crate::maps::*;
	use crate::tiles::*;

	pub fn generate(map: &mut Map) {
		rectangle(map, 1, 1, map.width()-1, map.height()-1, Tile::empty);

		// Room test.
		rectangle(map, 10, 10, 30, 20, Tile::wall);		// Wall.
		rectangle(map, 20, 10, 21, 11, Tile::empty);	// Door.
		rectangle(map, 11, 11, 29, 19, Tile::empty);	// Interior.
		rectangle(map, 15, 15, 18, 18, Tile::gold);		// Goooooold.
	}

	fn rectangle(map: &mut Map, x1: usize, y1: usize, x2: usize, y2: usize, t: fn() -> Tile) {
		for y in y1..y2 as usize {
			for x in x1..x2 as usize {
				map.set(x, y, t());
			}
		}
	}
}
