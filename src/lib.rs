mod actor;
mod ai;
mod dgen;
mod dogemath;
mod map;
mod skill;
mod tile;
pub use actor::actors;
pub use ai::ais;
pub use dgen::generator;
pub use dogemath::dogemaths;
pub use map::maps;
pub use skill::skills;
pub use tile::tiles;

pub mod dogestuff {
	use tcod::console::{Offscreen, Root};
	use tcod::input::{self, Event, Mouse};
	use tcod::map::Map as FovMap;
	pub struct Screen {
		pub root: Root,
		pub con: Offscreen,
		pub fov_map: FovMap,
		pub mouse: Mouse,
	}
	#[derive(PartialEq, Debug)]
	pub enum Actions {
		TookAction,
		NoAction,
		Exit,
	}
}
