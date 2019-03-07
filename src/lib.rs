mod actor;
mod dgen;
mod dogemath;
mod map;
mod skill;
mod tile;
pub use actor::actors;
pub use dgen::generator;
pub use dogemath::dogemaths;
pub use map::maps;
pub use skill::skills;
pub use tile::tiles;

pub mod screens {
	use tcod::console::{Offscreen, Root};
	use tcod::input::{self, Event, Mouse};
	pub struct Screen {
		pub root: Root,
		pub con: Offscreen,
		pub mouse: Mouse,
	}
}
