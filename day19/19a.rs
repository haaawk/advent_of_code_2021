use std::io::{self, BufRead};
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

fn compare(a: &HashSet::<[i32;3]>, b: &HashSet::<[i32;3]>) -> Option<HashSet::<[i32;3]>> {
	for x_pick in 0..3 {
		for y_pick in 0..3 {
			if x_pick == y_pick {
				continue;
			}
			for z_pick in 0..3 {
				if x_pick == z_pick || y_pick == z_pick {
					continue;
				}
				for x_shift in [-1, 1] {
					for y_shift in [-1, 1] {
						for z_shift in [-1, 1] {
							let mut dists = HashMap::<[i32;3], u32>::new();
							for aa in a {
								for bb in b {
									let d = [aa[0] - x_shift * bb[x_pick], aa[1] - y_shift * bb[y_pick], aa[2] - z_shift * bb[z_pick]];
									let new_count = 1 + dists.get(&d).unwrap_or(&0);
									dists.insert(d, new_count);
								}
							}
							for (k, v) in dists {
								if v >= 12 {
									let mut res = HashSet::<[i32;3]>::new();
									for bb in b {
										res.insert([k[0] + x_shift * bb[x_pick], k[1] + y_shift * bb[y_pick], k[2] + z_shift * bb[z_pick]]);
									}
									return Some(res);
								}
							}		
						}
					}
				}
			}
		}
	}
	None
}

fn main() {
	let stdin = io::stdin();
	let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
	let mut idx = 0;
	let mut s = Vec::<HashSet::<[i32;3]>>::new();
	while idx < lines.len() {
		idx += 1;
		let mut b = HashSet::<[i32;3]>::new();
		while idx < lines.len() && !lines[idx].trim().is_empty() {
			let nums: Vec<i32> = lines[idx].trim().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
			b.insert([nums[0], nums[1], nums[2]]);
			idx += 1;
		}
		s.push(b);
		idx += 1;
	}
	let mut b = s[0].clone();
	let mut q = VecDeque::<HashSet::<[i32;3]>>::new();
	for idx in 1..s.len() {
		q.push_back(s[idx].clone());
	}
	while let Some(bb) = q.pop_front() {
		let fit = compare(&b, &bb);
		match fit {
			Some(new_b) => b.extend(new_b),
			None => q.push_back(bb),
		};
	}
	println!("{}", b.len());
}
