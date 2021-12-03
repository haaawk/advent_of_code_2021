use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	let mut lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
	lines.sort();
	println!("{}", to_number(find(&lines, 0, |v, p| &v[p..], |v, p| &v[..p])) * to_number(find(&lines, 0, |v, p| &v[..p], |v, p| &v[p..])));
}

fn find(lines: &[String], level: usize, ones: fn(&[String], usize) -> &[String], zeros: fn(&[String], usize) -> &[String]) -> &String {
	if lines.len() == 1 {
		return &lines[0];
	}
	let x = lines.partition_point(|line| line.as_bytes()[level] == b'0');
	if x <= lines.len() - x {
		find(ones(lines, x), level + 1, ones, zeros)
	} else {
		find(zeros(lines, x), level + 1, ones, zeros)
	}
}

fn to_number(line: &String) -> u64 {
	let mut res: u64 = 0;
	let mut power: u64 = 1;
	for idx in 0..line.len() {
		res += power * (if line.as_bytes()[line.len() - idx - 1] == b'0' { 0 } else { 1 });
		power *= 2;
	}
	res
}
