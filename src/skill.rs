pub mod skills {
	use crate::actors::*;
	use crate::dogemaths::clamp;
	use crate::dogestuff::{mut_two, Actions, Screen};
	use crate::maps::*;
	use tcod::colors::{self, Color};
	use Actions::*;

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

		pub fn hit() -> Self {
			Skill {
				name: String::from("Hit your opponent"),
				cool_down: 1,
				cool_down_left: 0,
				skill: SkillTypes::hit,
			}
		}

		pub fn use_skill(
			skilltype: SkillTypes,
			id: usize,
			other_id: Option<usize>,
			dir: (i32, i32),
			val: i32,
			map: &Map,
			actors: &mut [Actor],
			screen: &mut Screen,
		) -> Actions {
			let skill_id = actors[id]
				.skills
				.iter()
				.position(|skill| skill.skill == skilltype);
			match skill_id {
				Some(skill_id) => {
					if actors[id].skills[skill_id].cool_down_left == 0 {
						let on_use: fn(
							usize,
							Option<usize>,
							(i32, i32),
							i32,
							&Map,
							&mut [Actor],
							&mut Screen,
						) -> Actions = match actors[id].skills[skill_id].skill {
							SkillTypes::move_attack => move_attack,
							SkillTypes::hit => hit,
						};
						let used = on_use(id, other_id, dir, val, map, actors, screen);
						if used == TookAction {
							actors[id].skills[skill_id].cool_down_left =
								actors[id].skills[skill_id].cool_down;
						}
						used
					} else {
						screen.messages.add_message(
							format!(
								"Skill can be used in {} turns",
								actors[id].skills[skill_id].cool_down_left
							),
							colors::LIGHT_AZURE,
						);
						NoAction
					}
				}
				None => NoAction,
			}
		}
	}

	fn hit(
		id: usize,
		other_id: Option<usize>,
		dir: (i32, i32),
		val: i32,
		map: &Map,
		actors: &mut [Actor],
		screen: &mut Screen,
	) -> Actions {
		match other_id {
			Some(other_id) => {
				let atk;
				let def;
				if let Some(stats) = &actors[id].stats.as_ref() {
					atk = stats.atk;
				} else {
					atk = 0;
				}
				if let Some(stats) = &actors[other_id].stats.as_ref() {
					def = stats.def;
				} else {
					def = 0;
				}
				let dmg = clamp(atk - def, 0, (atk - def).abs());
				screen.messages.add_message(
					format!(
						"{} hit {} for {} damage.",
						actors[id].name, actors[other_id].name, dmg
					),
					colors::LIGHT_RED,
				);
				if actors[other_id].take_damage(dmg, screen) {
					gain_exp(id, screen, actors);
				}
				TookAction
			}
			None => NoAction,
		}
	}

	fn gain_exp(id: usize, screen: &mut Screen, actors: &mut [Actor]) {
		if let Some(stats) = &mut actors[id].stats.as_mut() {
			stats.xp += 2;
			if stats.xp >= 10 {
				stats.xp = 0;
				let hp = (rand::random::<f32>() * 3.).ceil() as i32;
				let atk = (rand::random::<f32>() * 2.).ceil() as i32;
				let def = (rand::random::<f32>() * 2.).ceil() as i32;
				stats.max_hp += hp;
				stats.hp = stats.max_hp;
				stats.atk += atk;
				stats.def += def;
				screen.messages.add_message(
					format!(
						"You leveled up!, you gained +{} hp, +{} atk, +{} def",
						hp, atk, def
					),
					colors::GOLD,
				);
			}
		}
	}

	fn move_attack(
		id: usize,
		other_id: Option<usize>,
		dir: (i32, i32),
		val: i32,
		map: &Map,
		actors: &mut [Actor],
		screen: &mut Screen,
	) -> Actions {
		let (new_x, new_y) = (actors[id].x + (dir.0 * val), actors[id].y + (dir.1 * val));
		let target_id = actors
			.iter()
			.position(|actor| (actor.x, actor.y) == (new_x, new_y));
		match target_id {
			Some(target_id) => {
				if actors[target_id].alive {
					Skill::use_skill(
						actors[id].default_skill,
						id,
						Some(target_id),
						dir,
						val,
						map,
						actors,
						screen,
					)
				} else {
					move_by(id, None, dir, val, map, actors, screen)
				}
			}

			None => move_by(id, None, dir, val, map, actors, screen),
		}
	}

	fn move_by(
		id: usize,
		other_id: Option<usize>,
		dir: (i32, i32),
		val: i32,
		map: &Map,
		actors: &mut [Actor],
		screen: &mut Screen,
	) -> Actions {
		let (new_x, new_y) = (actors[id].x + (dir.0 * val), actors[id].y + (dir.1 * val));
		if Actor::is_blocked(actors, new_x, new_y)
			&& Map::is_blocked(map, new_x as usize, new_y as usize)
		{
			actors[id].x = new_x;
			actors[id].y = new_y;
			screen.messages.add_message(
				format!("{} moved to {},{}", actors[id].name, new_x, new_y),
				colors::LIGHTER_AZURE,
			);
			return TookAction;
		}
		NoAction
	}
}
