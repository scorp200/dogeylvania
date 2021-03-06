pub mod dogemaths {
	use rand;

	pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
		if value < min {
			min
		} else if value > max {
			max
		} else {
			value
		}
	}

	pub fn getDistanceRaw(o1: (f32, f32), o2: (f32, f32)) -> f32 {
		let x = o1.0 - o2.0;
		let y = o1.1 - o2.1;
		x * x + y * y
	}

	pub fn getDistance(o1: (f32, f32), o2: (f32, f32)) -> f32 {
		getDistanceRaw(o1, o2).sqrt()
	}

	pub fn percenttoPoint(a: f32, b: f32, p: f32) -> f32 {
		(p * (b - a) + a)
	}

	pub fn pointtoPercent(a: f32, b: f32, c: f32) -> f32 {
		(c - a) / (b - a)
	}

	pub fn shufflearray<T>(arr: &mut Vec<T>) {
		let mut currentIndex = arr.len();
		let mut randomIndex: usize;
		while 0 != currentIndex {
			randomIndex = (rand::random::<f32>() * currentIndex as f32).floor() as usize;
			currentIndex -= 1;
			arr.swap(currentIndex, randomIndex);
		}
	}

	pub mod Direction {
		pub const NORTH: (i32, i32) = (0, -1);
		pub const SOUTH: (i32, i32) = (0, 1);
		pub const EAST: (i32, i32) = (1, 0);
		pub const WEST: (i32, i32) = (-1, 0);

		pub const NORTHEAST: (i32, i32) = (1, -1);
		pub const NORTHWEST: (i32, i32) = (-1, -1);
		pub const SOUTHEAST: (i32, i32) = (1, 1);
		pub const SOUTHWEST: (i32, i32) = (-1, 1);
	}

	pub fn get_dir_towards(me: (f32, f32), target: (f32, f32)) -> (i32, i32) {
		let dx = target.0 - me.0;
		let dy = target.1 - me.1;
		let dist = getDistance(me, target);
		(
			(dx as f32 / dist).round() as i32,
			(dy as f32 / dist).round() as i32,
		)
	}

	pub struct Rect {
		pub x: i32,
		pub y: i32,
		pub w: i32,
		pub h: i32,
	}

	impl Rect {
		pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
			Rect {
				x: x,
				y: y,
				w: w,
				h: h,
			}
		}

		pub fn topleft(&self) -> (i32, i32) {
			(self.x, self.y)
		}

		pub fn topright(&self) -> (i32, i32) {
			(self.x + self.w, self.y)
		}

		pub fn bottomleft(&self) -> (i32, i32) {
			(self.x, self.y + self.h)
		}

		pub fn bottomright(&self) -> (i32, i32) {
			(self.x + self.w, self.y + self.h)
		}

		pub fn center(&self) -> (i32, i32) {
			(self.x + self.w / 2, self.y + self.h / 2)
		}

		pub fn intersects(&self, other: Rect) -> bool {
			self.topleft() < other.bottomright() && self.topright() > other.bottomleft()
		}

		pub fn pointinside(&self, point: (i32, i32)) -> bool {
			self.topleft() < point && self.topright() > point
		}
	}
}
