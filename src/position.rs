use std::cmp::PartialEq;

use rand;
use rand::Rng;

pub struct Position {
	pub x: u8,
	pub y: u8,
}

impl Position {
	pub fn random() -> Position {
		let mut rng = rand::thread_rng();

		let x: u8 = rng.gen_range(1, ::COLS as u8);
		let y: u8 = rng.gen_range(1, ::ROWS as u8);

		Position { x: x, y: y}
	}
}

impl PartialEq for Position {
	fn eq(&self, other: &Position) -> bool {
		self.x == other.x && self.y == other.y
	}
}