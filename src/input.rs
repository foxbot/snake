use std::io;
use std::io::Read;

use ::Direction::*;

const KEY_W: u8 = 0x77;
const KEY_S: u8 = 0x73;
const KEY_A: u8 = 0x61;
const KEY_D: u8 = 0x64;

pub struct Input {
	pub dir: ::Direction,
}

impl Input {
	pub fn new() -> Input {
		Input {
			dir: Weast,
		}
	}

	pub fn run_loop(&mut self) {
		let mut stdin = io::stdin();

		loop {
			let mut buf = [0u8; 1];
			let count = stdin.read(&mut buf[..]).unwrap();
			
			let dir = match buf[0] {
				KEY_W => North,
				KEY_S => South,
				KEY_A => East,
				KEY_D => West,
				x => { println!("unknown keycode: {}", x); Weast }
			};
			self.dir = dir;
		}
	}
}