use std::io::{self, BufRead};
use std::cmp::min;

fn main() {
	let stdin = io::stdin();
	let lines = stdin.lock().lines().map(|l| l.unwrap());
	let mut b = [[u64::MAX; 101]; 101];
	let mut row = 0;
	for line in lines {
		row += 1;
		let mut col = 0;
		for c in line.as_bytes() {
			col += 1;
			b[row][col] = (c - b'0') as u64;
		}
		
	}
	b[0][1] = 0;
	b[1][0] = 0;
	b[1][1] = 0;
	for row in 1..101 {
		for col in 1..101 {
			b[row][col] += min(b[row - 1][col], b[row][col - 1]);
		}
	}
	println!("{}", b[100][100]);
}
