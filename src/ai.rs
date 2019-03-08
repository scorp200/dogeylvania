pub mod ais {
	use crate::actors::*;
	use crate::dogemaths::Direction;
	use crate::dogestuff::{Actions, Screen};
	use crate::maps::*;
	use Actions::*;
	#[derive(PartialEq, Debug)]
	pub struct Ai;

	impl Ai {
		pub fn take_turn(
			id: usize,
			dir: (i32, i32),
			val: i32,
			map: &Map,
			actors: &mut [Actor],
			screen: &mut Screen,
		) {
		}
	}
}
