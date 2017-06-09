use std::io;
use std::io::Write;
use std::string;

use nix::sys::termios;

// characters appear immediately into stdin
// github.com/geofft/demo-rust-getch

pub fn configure() {
	let saved_term = termios::tcgetattr(0).unwrap();
	let mut term = saved_term;

	term.c_lflag.remove(termios::ICANON);
	//term.c_lflag.remove(termios::ISIG);
	term.c_lflag.remove(termios::ECHO);
	termios::tcsetattr(0, termios::TCSADRAIN, &term).unwrap();
}

pub fn jump() {
	let mut stdout = io::stdout();
	let buffer = String::from("\x1B[1;1H").into_bytes();
	stdout.write(&buffer).unwrap();
	stdout.flush().unwrap();
}
pub fn erase() {
	let mut stdout = io::stdout();
	let buffer = String::from("\x1B[2J").into_bytes();
	stdout.write(&buffer).unwrap();
	stdout.flush().unwrap();
}