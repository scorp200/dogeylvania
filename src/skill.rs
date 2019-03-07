pub mod skills {
	use crate::actors::*;
	use crate::dogemaths::Direction;
	use crate::maps::*;
	use crate::screens::Screen;

	#[derive(Clone, Copy, PartialEq, Debug)]
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
			skilltype: SkillTypes,
			id: usize,
			dir: (i32, i32),
			val: i32,
			map: &Map,
			actors: &mut [Actor],
			screen: &mut Screen,
		) -> bool {
			let skill_id = actors[id]
				.skills
				.iter()
				.position(|skill| skill.skill == skilltype);
			match skill_id {
				Some(skill_id) => {
					if actors[id].skills[skill_id].cool_down_left == 0 {
						let on_use: fn(
							usize,
							(i32, i32),
							i32,
							&Map,
							&mut [Actor],
							&mut Screen,
						) -> bool = match actors[id].skills[skill_id].skill {
							SkillTypes::move_attack => move_by,
							SkillTypes::hit => move_attack,
						};
						let used = on_use(id, dir, val, map, actors, screen);
						if used {
							actors[id].skills[skill_id].cool_down_left =
								actors[id].skills[skill_id].cool_down;
						}
						used
					} else {
						println!(
							"Skill can be used in {} turns",
							actors[id].skills[skill_id].cool_down_left
						);
						false
					}
				}
				None => false,
			}
		}
	}

	pub fn hit(
		id: usize,
		dir: ((i32, i32), i32),
		map: &Map,
		actors: &mut [Actor],
		screen: &mut Screen,
	) -> bool {
		println!("hitting someone...");
		true
	}

	pub fn move_attack(
		id: usize,
		dir: (i32, i32),
		val: i32,
		map: &Map,
		actors: &mut [Actor],
		screen: &mut Screen,
	) -> bool {
		let (new_x, new_y) = (actors[id].x + (dir.0 * val), actors[id].y + (dir.1 * val));
		let target_id = actors
			.iter()
			.position(|actor| (actor.x, actor.y) == (new_x, new_y));
		match target_id {
			Some(target_id) => {
				Skill::use_skill(actors[id].default_skill, id, dir, val, map, actors, screen)
			}
			None => move_by(id, dir, val, map, actors, screen),
		}
	}

	pub fn move_by(
		id: usize,
		dir: (i32, i32),
		val: i32,
		map: &Map,
		actors: &mut [Actor],
		screen: &mut Screen,
	) -> bool {
		let (new_x, new_y) = (actors[id].x + (dir.0 * val), actors[id].y + (dir.1 * val));
		if Map::is_blocked(map, new_x as usize, new_y as usize) {
			actors[id].x = new_x;
			actors[id].y = new_y;
			println!("{} moved to {},{}", actors[id].name, new_x, new_y);
			return true;
		}
		false
	}
}
