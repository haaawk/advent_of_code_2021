use std::io::{self, BufRead};
use std::collections::VecDeque;
use std::cmp::min;

fn main() {
	let stdin = io::stdin();
	let lines = stdin.lock().lines().map(|l| l.unwrap());
	let mut b = [[u64::MAX; 501]; 501];
	let mut best = [[u64::MAX; 501]; 501];
	let mut row = 0;
	for line in lines {
		row += 1;
		let mut col = 0;
		for c in line.as_bytes() {
			col += 1;
			for x in 0..5 {
				for y in 0..5 {
					let mut val = (c - b'0') as u64 + x as u64 + y as u64;
					if val > 9 {
						val -= 9;
					}
					b[x * (b.len() - 1) / 5 + row][y * (b[0].len() - 1) / 5 + col] = val;
				}
			}
		}
		
	}
	b[1][1] = 0;
	best[1][1] = 0;
	let mut res = u64::MAX;
	let mut q = VecDeque::<(usize, usize)>::new();
	q.push_back((1, 1));
	while let Some((x, y)) = q.pop_front() {
		if x == b.len() - 1 && y == b[0].len() - 1 {
			res = min(res, best[x][y]);
		} else {
			if x > 1 && best[x - 1][y] > best[x][y] + b[x - 1][y] {
				best[x - 1][y] = best[x][y] + b[x - 1][y];
				q.push_back((x - 1, y));
			}
			if y > 1 && best[x][y - 1] > best[x][y] + b[x][y - 1] {
				best[x][y - 1] = best[x][y] + b[x][y - 1];
				q.push_back((x, y - 1));
			}
			if x + 1 < b.len() && best[x + 1][y] > best[x][y] + b[x + 1][y] {
				best[x + 1][y] = best[x][y] + b[x + 1][y];
				q.push_back((x + 1, y));
			}
			if y + 1 < b[0].len() && best[x][y + 1] > best[x][y] + b[x][y + 1] {
				best[x][y + 1] = best[x][y] + b[x][y + 1];
				q.push_back((x, y + 1));
			}
		}
	}
	
	println!("{}", res);
}
