pub mod skills {
	use crate::actors::*;
	use crate::dogemaths::Direction;
	use crate::maps::*;
	use crate::screens::Screen;

	#[derive(PartialEq, Debug)]
	pub enum SkillTypes {
		move_attack,
		hit,
	}

	#[derive(PartialEq, Debug)]
	pub struct Skill {
		pub name: String,
		cool_down: u8,
		pub cool_down_left: u8,
		pub skill: SkillTypes,
	}

	impl Skill {
		pub fn move_attack() -> Self {
			Skill {
				name: String::from("Move or attack"),
				cool_down: 1,
				cool_down_left: 0,
				skill: SkillTypes::move_attack,
			}
		}

		pub fn use_skill(
			&mut self,
			id: usize,
			dir: ((i32, i32), i32),
			map: &Map,
			actors: &mut [Actor],
			screen: &mut Screen,
		) {
			if self.cool_down_left == 0 {
				let on_use: fn(usize, ((i32, i32), i32), &Map, &mut [Actor], &mut Screen) -> bool =
					match self.skill {
						SkillTypes::move_attack => move_by,
						SkillTypes::hit => move_attack,
					};
				if on_use(id, dir, map, actors, screen) {
					self.cool_down_left = self.cool_down;
				}
			}
		}
	}

	pub fn move_attack(
		id: usize,
		dir: ((i32, i32), i32),
		map: &Map,
		actors: &mut [Actor],
		screen: &mut Screen,
	) -> bool {
		let (new_x, new_y) = (
			actors[id].x + ((dir.0).0 * dir.1),
			actors[id].y + ((dir.0).1 * dir.1),
		);
		let target_id = actors
			.iter()
			.position(|actor| (actor.x, actor.y) == (new_x, new_y));
		match target_id {
			Some(target_id) => {
				println!("attacking...");
				true
			}
			None => move_by(id, dir, map, actors, screen),
		}
	}

	pub fn move_by(
		id: usize,
		dir: ((i32, i32), i32),
		map: &Map,
		actors: &mut [Actor],
		screen: &mut Screen,
	) -> bool {
		println!("moving to {},{}", (dir.0).0, (dir.0).1);
		true
	}
}
