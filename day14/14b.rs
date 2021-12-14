use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
	let stdin = io::stdin();
	let mut collecting_rules = false;
	let mut rules = HashMap::<(u8, u8), u8>::new();
	let mut q = HashMap::<(u8, u8), u64>::new();
	let mut first = 0u8;
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
			first = line.as_bytes()[0];
			for idx in 1..line.as_bytes().len() {
				let p = (line.as_bytes()[idx - 1], line.as_bytes()[idx]);
				if let Some(x) = q.get_mut(&p) {
					*x += 1;
				} else {
					q.insert(p, 1);
				}
			}
		}
	}
	for _step in 0..40 {
		println!("step {}", _step);
		let mut qq = HashMap::<(u8, u8), u64>::new();
		for (a, b) in q.iter() {
			let m = rules.get(&(a.0, a.1)).unwrap();
			let p = (a.0, *m);
			let pp = (*m, a.1);
			if let Some(x) = qq.get_mut(&p) {
				*x += *b;
			} else {
				qq.insert(p, *b);
			}
			if let Some(x) = qq.get_mut(&pp) {
				*x += *b;
			} else {
				qq.insert(pp, *b);
			}
		}
		q = qq;
	}
	let mut count = HashMap::<u8, u64>::new();
	count.insert(first, 1);
	for (a, b) in q.iter() {
		if let Some(x) = count.get_mut(&a.1) {
			*x += *b;
		} else {
			count.insert(a.1, *b);
		}
	}
	let min = count.values().min().unwrap();
	let max = count.values().max().unwrap();
	println!("{}", max - min);
}
