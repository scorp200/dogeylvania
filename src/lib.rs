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
	use std::cmp;
	use tcod::colors::{self, Color};
	use tcod::console::{Offscreen, Root};
	use tcod::input::{self, Event, Mouse};
	use tcod::map::Map as FovMap;
	pub struct Screen {
		pub root: Root,
		pub con: Offscreen,
		pub fov_map: FovMap,
		pub fov_enable: bool,
		pub last_fov: bool,
		pub messages: MSG,
	}

	pub struct Ui {
		pub ui: Offscreen,
		pub msg: Offscreen,
	}

	#[derive(PartialEq, Debug)]
	pub enum Actions {
		TookAction,
		NoAction,
		Exit,
	}

	const MAX_MSG: usize = 15;

	pub struct MSG {
		pub msg: Vec<(String, Color)>,
	}

	impl MSG {
		pub fn add_message(&mut self, msg: String, color: Color) {
			self.msg.insert(0, (msg, color));
			if self.msg.len() > MAX_MSG {
				self.msg.pop();
			}
		}
	}

	pub fn mut_two<T>(
		first_index: usize,
		second_index: usize,
		items: &mut [T],
	) -> (&mut T, &mut T) {
		assert!(first_index != second_index);
		let split_at_index = cmp::max(first_index, second_index);
		let (first_slice, second_slice) = items.split_at_mut(split_at_index);
		if first_index < second_index {
			(&mut first_slice[first_index], &mut second_slice[0])
		} else {
			(&mut second_slice[0], &mut first_slice[second_index])
		}
	}
}
