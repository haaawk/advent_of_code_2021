use std::io::{self, BufRead};
use std::collections::{BTreeMap, VecDeque};

fn main() {
	let stdin = io::stdin();
	let mut collecting_rules = false;
	let mut rules = BTreeMap::<(u8, u8), u8>::new();
	let mut q = VecDeque::<(u8, u8)>::new();
	for line_opt in stdin.lock().lines() {
		let line = line_opt.unwrap();
		if line.is_empty() {
			collecting_rules = true;
		} else if collecting_rules {
			let mut split = line.split(" -> ");
			let x = split.next().unwrap();
			let y = split.next().unwrap();
			rules.insert((x.as_bytes()[0], x.as_bytes()[1]), y.as_bytes()[0]);	
		} else {
			for idx in 1..line.as_bytes().len() {
				q.push_back((line.as_bytes()[idx - 1], line.as_bytes()[idx]));
			}
		}
	}
	for _step in 0..10 {
		let mut qq = VecDeque::<(u8, u8)>::new();
		while let Some((a, b)) = q.pop_front() {
			let m = rules.get(&(a, b)).unwrap();
			qq.push_back((a, *m));
			qq.push_back((*m, b));
		}
		q = qq;
	}
	let mut count = BTreeMap::<u8, u64>::new();
	count.insert(q.front().unwrap().0, 1);
	while let Some((_a, b)) = q.pop_front() {
		if let Some(x) = count.get_mut(&b) {
			*x += 1;
		} else {
			count.insert(b, 1);
		}
	}
	let min = count.values().min().unwrap();
	let max = count.values().max().unwrap();
	println!("{}", max - min);
}
