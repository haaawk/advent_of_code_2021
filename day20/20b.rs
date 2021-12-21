use std::io::{self, BufRead};

fn to_index(b: &[[bool; 206]; 206], x: usize, y: usize) -> usize {
	let mut res = 0;
	for dx in 0..3 {
		for dy in 0..3 {
			res = 2 * res + (if b[x + dx - 1][y + dy - 1] { 1 } else { 0 });
		}
	}
	res
}

fn main() {
	let stdin = io::stdin();
	let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap().trim().to_string()).collect();

	let mut algo = Vec::<bool>::with_capacity(lines[0].as_bytes().len());
	for c in lines[0].as_bytes() {
		algo.push(if *c == b'#' { true } else { false });
	}

	let mut b = [[false; 206]; 206];
	
	for row in 0..lines.len() - 2 {
		for col in 0..lines[2].len() {
			b[row + 53][col + 53] = lines[row + 2].as_bytes()[col] == b'#';
		}
	}

	for _step in 0..25 {
		// Step 1 - edge positions are all '.'
		let mut b2 = [[true; 206]; 206];
		for row in 1..b.len() - 1 {
			for col in 1..b[0].len() - 1 {
				b2[row][col] = algo[to_index(&b, row, col)];	
			}
		}

		// Step 2 - edge positions are all '#'
		let mut b3 = [[false; 206]; 206];
		for row in 1..b.len() - 1 {
			for col in 1..b[0].len() - 1 {
				b3[row][col] = algo[to_index(&b2, row, col)];
			}
		}
		b = b3;
	}
	let mut res = 0u64;
	for row in 0..b.len() {
		for col in 0..b[0].len() {
			res += if b[row][col] { 1 } else { 0 };
		}
	}
	println!("{}", res);
}
