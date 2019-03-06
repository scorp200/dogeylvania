pub mod generator {
	use crate::maps::*;
	use crate::tiles::*;

	pub fn generate(map: &mut Map) {
		for y in 1..map.height()-1 as usize {
    	    for x in 1..map.width()-1 as usize {
				map.set(x, y, Tile::empty());
    		}
    	}
	}
}
