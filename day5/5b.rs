use std::io::{self, BufRead};

fn parse_point(val: &str) -> (usize, usize) {
	let mut split = val.split(",");
	(split.next().unwrap().parse::<usize>().unwrap(), split.next().unwrap().parse::<usize>().unwrap())
}

fn main() {
	let stdin = io::stdin();
	let lines = stdin.lock().lines().map(|l| l.unwrap());
	let mut b = [[0; 1000]; 1000];
	for line in lines {
		let mut split = line.split(" -> ");
		let (mut x1, mut y1) = parse_point(split.next().unwrap());
		let (x2, y2) = parse_point(split.next().unwrap());
		let dx = if x1 > x2 { -1 } else if x1 < x2 { 1 } else { 0 };
		let dy = if y1 > y2 { -1 } else if y1 < y2 { 1 } else { 0 };
		while x1 != x2 || y1 != y2 {
			b[x1][y1] += 1;
			x1 = (x1 as i32 + dx) as usize;
			y1 = (y1 as i32 + dy) as usize;
		}
		b[x2][y2] += 1;
	}
	println!("{}", b.iter().map(|row| row.iter().filter(|x| **x > 1).count()).sum::<usize>());
}
