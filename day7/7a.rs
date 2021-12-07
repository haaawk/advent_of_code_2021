use std::io;
use std::cmp::min;

fn main() {
	let mut line = String::new();
	io::stdin().read_line(&mut line).unwrap();
	let mut p: Vec<usize> = line.trim().split(",").map(|x| x.parse::<usize>().unwrap()).collect();
	p.sort();
	let mut left = vec![0; p.len()];
	let mut right = vec![0; p.len()];
	for idx in 1..p.len() {
		left[idx] = left[idx - 1] + idx * (p[idx] - p[idx - 1]);
		right[p.len() - idx - 1] = right[p.len() - idx] + idx * (p[p.len() - idx] - p[p.len() - idx - 1]);
	}
	let mut res = usize::MAX;
	for idx in 0..p.len() {
		res = min(res, left[idx] + right[idx]);
	}
	println!("{}", res);
}
