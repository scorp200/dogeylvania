pub mod ais {
	use crate::actors::Actor;
	use crate::dogemaths::*;
	use crate::dogestuff::Screen;
	use crate::maps::*;
	use crate::skills::*;
	use tcod::colors::{self, Color};
	use tcod::console::*;
	use tcod::map::Map as FovMap;

	#[derive(PartialEq, Debug)]
	pub struct Ai;

	impl Ai {
		pub fn take_turn(id: usize, map: &Map, actors: &mut [Actor], screen: &mut Screen) {
			if screen.fov_map.is_in_fov(actors[id].x, actors[id].y) {
				let (player_x, player_y) = (actors[0].x, actors[0].y);
				let dir = get_dir_towards(
					(actors[id].x as f32, actors[id].y as f32),
					(player_x as f32, player_y as f32),
				);
				Skill::use_skill(
					SkillTypes::move_attack,
					id,
					None,
					dir,
					1,
					map,
					actors,
					screen,
				);
			}
		}
	}
}
