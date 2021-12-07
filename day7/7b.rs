use std::io;
use std::cmp::min;

fn main() {
	let mut line = String::new();
	io::stdin().read_line(&mut line).unwrap();
	let p: Vec<i64> = line.trim().split(",").map(|x| x.parse::<i64>().unwrap()).collect();
	let start = *p.iter().min().unwrap();
	let end = *p.iter().max().unwrap();
	let mut res = i64::MAX;
	for idx in start..=end {
		let mut cost = 0;
		for idx2 in 0..p.len() {
			let d = (idx - p[idx2]).abs();
			cost += (d * (d + 1)) / 2;
		}
		res = min(res, cost);
	}
	println!("{}", res);
}
