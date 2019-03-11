pub mod actors {
	use crate::ais::Ai;
	use crate::dogestuff::Screen;
	use crate::skills::*;
	use tcod::colors::{self, Color};
	use tcod::console::*;

	pub struct Actor {
		pub x: i32,
		pub y: i32,
		pub char: char,
		pub color: Color,
		pub name: String,
		pub skills: Vec<Skill>,
		pub default_skill: SkillTypes,
		pub alive: bool,
		pub block_move: bool,
		pub ai: Option<Ai>,
		pub stats: Option<Stats>,
	}

	pub struct Stats {
		pub hp: i32,
		pub max_hp: i32,
		pub atk: i32,
		pub def: i32,
		pub xp: i32,
		pub on_death: DeathCallBack,
	}

	pub enum DeathCallBack {
		Player,
		Monster,
	}

	fn player_death(actor: &mut Actor, screen: &mut Screen) {
		screen
			.messages
			.add_message(format!("{} died", actor.name), colors::CRIMSON);
		screen
			.messages
			.add_message(format!("Press R to restart."), colors::CRIMSON);
		actor.char = '%';
		actor.color = colors::DARK_RED;
	}

	fn monster_death(actor: &mut Actor, screen: &mut Screen) {
		println!("{} has died", actor.name);
		actor.char = '%';
		actor.color = colors::DARK_RED;
		actor.block_move = false;
		actor.stats = None;
		actor.ai = None;
	}

	impl Actor {
		pub fn new(
			x: i32,
			y: i32,
			char: char,
			color: Color,
			name: String,
			alive: bool,
			block: bool,
		) -> Self {
			Actor {
				x: x,
				y: y,
				char: char,
				color: color,
				name: name,
				alive: alive,
				skills: Vec::default(),
				default_skill: SkillTypes::hit,
				block_move: block,
				ai: None,
				stats: None,
			}
		}

		pub fn draw(&self, screen: &mut Screen) {
			screen.con.set_default_foreground(self.color);
			screen
				.con
				.put_char(self.x, self.y, self.char, BackgroundFlag::None);
		}

		pub fn clear(&self, screen: &mut Screen) {
			screen.con.set_default_foreground(self.color);
			screen
				.con
				.put_char(self.x, self.y, ' ', BackgroundFlag::None);
		}

		pub fn is_blocked(actors: &mut [Actor], x: i32, y: i32) -> bool {
			!actors
				.iter()
				.any(|actor| actor.block_move && actor.x == x && actor.y == y)
		}

		pub fn take_damage(&mut self, dmg: i32, screen: &mut Screen) -> bool {
			if let Some(stats) = self.stats.as_mut() {
				if dmg > 0 {
					stats.hp -= dmg;
					screen.messages.add_message(
						format!("{} has now {} hp.", self.name, stats.hp),
						colors::LIGHT_AMBER,
					);
				}

				if stats.hp <= 0 {
					self.alive = false;
					match stats.on_death {
						DeathCallBack::Player => {
							player_death(self, screen);
						}
						DeathCallBack::Monster => {
							monster_death(self, screen);
						}
					}
					return true;
				}
			}
			false
		}
	}
}
