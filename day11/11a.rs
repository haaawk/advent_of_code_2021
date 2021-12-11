use std::io::{self, BufRead};

fn flash(b: &mut [[i64; 12]; 12], row: usize, col: usize) -> u64 {
	if b[row][col] == 0 {
		return 0;
	}
	b[row][col] += 1;
	if b[row][col] > 9 {
		b[row][col] = 0;
		let mut res = 1u64;
		for dr in 0..3 {
			for dc in 0..3 {
				res += flash(b, row + dr - 1, col + dc - 1);
			}
		}
		return res;
	} else {
		return 0;
	}
}

fn main() {
	let stdin = io::stdin();
	let lines = stdin.lock().lines().map(|l| l.unwrap());
	let mut b = [[i64::MIN; 12]; 12];
	let mut row = 0;
	for line in lines {
		row += 1;
		let mut col = 0;
		for c in line.as_bytes() {
			col += 1;
			b[row][col] = (c - b'0') as i64;
		}
		
	}
	let mut res:u64 = 0;
	for _step in 0..100 {
		for row in 1..b.len() - 1 {
			for col in 1..b[0].len() - 1 {
				b[row][col] += 1;
			}
		}
		for row in 1..b.len() - 1 {
			for col in 1..b[0].len() - 1 {
				if b[row][col] > 9 {
					b[row][col] = 0;
					res += 1;
					for dr in 0..3 {
						for dc in 0..3 {
							res += flash(&mut b, row + dr - 1, col + dc - 1);
						}
					}
				}
			}
		}
	}
	println!("{}", res);
}
