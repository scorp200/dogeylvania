pub mod actors {
	use crate::ais::Ai;
	use crate::dogemaths::*;
	use crate::dogestuff::Screen;
	use crate::maps::*;
	use crate::skills::*;
	use tcod::colors::{self, Color};
	use tcod::console::*;
	use tcod::map::Map as FovMap;

	#[derive(PartialEq, Debug)]
	pub struct Actor {
		pub x: i32,
		pub y: i32,
		pub char: char,
		pub color: Color,
		pub name: String,
		pub skills: Vec<Skill>,
		pub default_skill: SkillTypes,
		pub block_move: bool,
		pub ai: Option<Ai>,
	}

	impl Actor {
		pub fn new(x: i32, y: i32, char: char, color: Color, name: String, block: bool) -> Self {
			Actor {
				x: x,
				y: y,
				char: char,
				color: color,
				name: name,
				skills: Vec::default(),
				default_skill: SkillTypes::hit,
				block_move: block,
				ai: None,
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
	}
}
