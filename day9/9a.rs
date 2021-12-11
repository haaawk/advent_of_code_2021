use std::io::{self, BufRead};
use std::cmp::min;

fn main() {
	let stdin = io::stdin();
	let lines = stdin.lock().lines().map(|l| l.unwrap());
	let mut b = [[10u8; 102]; 102];
	let mut row = 0;
	for line in lines {
		row += 1;
		let mut col = 0;
		for c in line.as_bytes() {
			col += 1;
			b[row][col] = c - b'0';
		}
		
	}
	let mut res:u64 = 0;
	for row in 1..b.len() - 1 {
		for col in 1..b[0].len() - 1 {
			if b[row][col] < min(min(b[row - 1][col], b[row + 1][col]), min(b[row][col - 1], b[row][col + 1])) {
				res += b[row][col] as u64 + 1;
			}
		}
	}
	println!("{}", res);
}
