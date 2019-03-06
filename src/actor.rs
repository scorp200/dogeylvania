pub mod actors {
	use crate::dogemaths::*;
	use crate::maps::*;
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
	}

	impl Actor {
		pub fn new(x: i32, y: i32, char: char, color: Color, name: String) -> Self {
			Actor {
				x: x,
				y: y,
				char: char,
				color: color,
				name: name,
			}
		}

		pub fn move_by(id: usize, x: i32, y: i32, map: &Map, actors: &mut [Actor]) {
			let (dx,dy) = (actors[id].y + x, actors[id].y + y);
			
		}

		pub fn draw(&self, con: &mut Console) {
			con.set_default_foreground(self.color);
			con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
		}
	}
}
