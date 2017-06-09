	extern crate nix;
extern crate rand;

mod console;
mod position;
mod input;

use std::io;
use std::io::Write;
use std::sync::Arc;
use std::thread;
use std::time;

use input::Input;
use position::Position;

fn main() {
	console::configure();
    let mut Game = Game::new();
    Game.run();
}

const COLS: usize = 48;
const ROWS: usize = 16;
const NEWLINE: [u8; 1] = [0xA; 1];

const BORDER: u8 = 0x23;	// #
const SPACE: u8	= 0x20;		// SPACE
const PLAYER: u8 = 0x2E;	// .
const TAIL: u8 = 0x2C;		// ,
const MANGO: u8 = 0x40;		// @

struct Game {
	char_dir: Direction,
	char_pos: Position,
	mango_pos: Position,
	score: u8,
	board: [[u8; COLS]; ROWS],
	refresh_rate: u64,
}

pub enum Direction {
	Weast,
	North,
	East,
	South,
	West
}

impl Game {
	fn new() -> Game {
		Game {
			char_dir: Direction::Weast,
			char_pos: Position { x: (COLS/2) as u8, y: (ROWS/2) as u8 },
			mango_pos: Position { x:0, y:0 },
			score: 0,
			board: [[SPACE; COLS]; ROWS],
			refresh_rate: 1,
		}
	}

	fn run(&mut self) {
		self.rand_mango();
		let input = Arc::new(Input::new());

		let input_thr = input.get_mut();
		thread::spawn(move || {	
			input_thr.run_loop();
		});
		let input_game = input.clone();

		'Game: loop  {
			self.char_dir = input.dir;
			self.do_move();
			self.fill_board();
			self.draw_board();
			thread::sleep(time::Duration::from_millis(1000 / self.refresh_rate));
		}
	}

	fn do_move(&mut self) {
		let vel = match self.char_dir {
			Direction::Weast	=> (0, 0),
			Direction::North	=> (0, 1),
			Direction::South	=> (0, -1),
			Direction::East		=> (1, 0),
			Direction::West		=> (-1, 0),
		};

		self.char_pos.x = {
			let mut x = self.char_pos.x as i8;
			x += vel.0;
			x as u8
		};
		self.char_pos.y = {
			let mut y = self.char_pos.y as i8;
			y += vel.1;
			y as u8
		};
	}

	fn fill_board(&mut self) {
		// Clear board
		self.board = [[SPACE; COLS]; ROWS];

		// Draw border

		let y_border = [BORDER; COLS];
		self.board[0] = y_border;
		self.board[ROWS - 1] = y_border;
		for y in 1..ROWS-1 {
			self.board[y][0] = BORDER;
			self.board[y][COLS-1] = BORDER;
		}

		// Draw mango
		let mango = &self.mango_pos;
		self.board[mango.y as usize][mango.x as usize] = MANGO;

		// Draw player
		let player = &self.char_pos;
		self.board[player.y as usize][player.x as usize] = PLAYER;
	}

	fn draw_board(&self) {
		console::erase();
		console::jump();
		let mut stdout = io::stdout();

		for y in 0..ROWS {
			stdout.write(&self.board[y]).unwrap();
			stdout.write(&NEWLINE).unwrap();
		}
		stdout.flush().unwrap();
		println!("score: {}", self.score)
	}

	fn rand_mango(&mut self) {
		
		let pos = Position::random();
		if pos == self.char_pos {
			self.rand_mango();
		} else {
			self.mango_pos = pos;
		}
	}
}