pub mod generator {
	use crate::maps::*;
	use crate::tiles::*;

	pub fn generate(map: &mut Map) {
		map.set(5, 5, Tile::empty());
	}
}
