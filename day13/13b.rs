use std::io::{self, BufRead};
use std::collections::BTreeSet;

fn main() {
	let stdin = io::stdin();
	let mut collecting_points = true;
	let mut points = Vec::<(i64, i64)>::new();
	for line_opt in stdin.lock().lines() {
		let line = line_opt.unwrap();
		if line.is_empty() {
			collecting_points = false;
		} else if collecting_points {
			let mut split = line.split(",");
			let x = split.next().unwrap().parse::<i64>().unwrap();
			let y = split.next().unwrap().parse::<i64>().unwrap();
			points.push((x, y));	
		} else {
			let mut split = line.split_whitespace();
			split.next();
			split.next();
			let mut split2 = split.next().unwrap().split("=");
			let direction = split2.next().unwrap();
			let number = split2.next().unwrap().parse::<i64>().unwrap();
			match direction {
				"x" => {
					for p in points.iter_mut() {
						if p.0 > number {
							p.0 = number - (p.0 - number);
						}
					}
				},
				"y" => {
					for p in points.iter_mut() {
						if p.1 > number {
							p.1 = number - (p.1 - number);
						}
					}
				},
				_ => panic!("Unexpected direction {}", direction),
			};
		}
	}
	let set = points.iter().cloned().collect::<BTreeSet<(i64, i64)>>();
	let max_x = set.iter().map(|x| x.0).max().unwrap();
	let max_y = set.iter().map(|x| x.1).max().unwrap();
	for row in 0..=max_y {
		for col in 0..=max_x {
			print!("{}", if set.contains(&(col, row)) { "X" } else { "." });
		}
		println!("");
	}
}
