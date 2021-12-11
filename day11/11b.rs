use std::io::{self, BufRead};

fn flash(b: &mut [[i64; 12]; 12], row: usize, col: usize) {
	if b[row][col] == 0 {
		return;
	}
	b[row][col] += 1;
	if b[row][col] > 9 {
		b[row][col] = 0;
		for dr in 0..3 {
			for dc in 0..3 {
				flash(b, row + dr - 1, col + dc - 1);
			}
		}
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
	let mut step = 0;
	loop {
		step += 1;
		for row in 1..b.len() - 1 {
			for col in 1..b[0].len() - 1 {
				b[row][col] += 1;
			}
		}
		for row in 1..b.len() - 1 {
			for col in 1..b[0].len() - 1 {
				if b[row][col] > 9 {
					b[row][col] = 0;
					for dr in 0..3 {
						for dc in 0..3 {
							flash(&mut b, row + dr - 1, col + dc - 1);
						}
					}
				}
			}
		}
		let mut finished = true;
		for row in 1..b.len() - 1 {
			for col in 1..b[0].len() - 1 {
				if b[row][col] != 0 {
					finished = false;
					break;
				}
			}
		}
		if finished {
			println!("{}", step);
			break;
		}
	}
}
