use std::io::{self, BufRead};
use std::cmp::min;

fn find_size(b: &mut [[u8; 102]; 102], row: usize, col: usize) -> u64 {
	b[row][col] = 9;
	let mut res = 1;
	if b[row - 1][col] != 9 {
		res += find_size(b, row - 1, col);
	}
	if b[row + 1][col] != 9 {
		res += find_size(b, row + 1, col);
	}
	if b[row][col - 1] != 9 {
		res += find_size(b, row, col - 1);
	}
	if b[row][col + 1] != 9 {
		res += find_size(b, row, col + 1);
	}
	return res;
}

fn main() {
	let stdin = io::stdin();
	let lines = stdin.lock().lines().map(|l| l.unwrap());
	let mut b = [[9u8; 102]; 102];
	let mut row = 0;
	for line in lines {
		row += 1;
		let mut col = 0;
		for c in line.as_bytes() {
			col += 1;
			b[row][col] = c - b'0';
		}
		
	}
	let mut res1:u64 = 0;
	let mut res2:u64 = 0;
	let mut res3:u64 = 0;
	for row in 1..b.len() - 1 {
		for col in 1..b[0].len() - 1 {
			if b[row][col] < min(min(b[row - 1][col], b[row + 1][col]), min(b[row][col - 1], b[row][col + 1])) {
				let basin_size = find_size(&mut b, row, col);
				if basin_size > res1 {
					res3 = res2;
					res2 = res1;
					res1 = basin_size;
				} else if basin_size > res2 {
					res3 = res2;
					res2 = basin_size;
				} else if basin_size > res3 {
					res3 = basin_size;
				}
			}
		}
	}
	println!("{}", res1 * res2 * res3);
}
