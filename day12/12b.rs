use std::io::{self, BufRead};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

fn main() {
	let stdin = io::stdin();
	let lines = stdin.lock().lines().map(|l| l.unwrap());
	let mut g = BTreeMap::<String, BTreeSet<String>>::new();
	for line in lines {
		let mut split = line.split("-");
		let end_a = split.next().unwrap().to_string();
		let end_b = split.next().unwrap().to_string();
		if let Some(ref mut s) = g.get_mut(&end_a) {
			s.insert(end_b.clone());
		} else {
			let mut s = BTreeSet::new();
			s.insert(end_b.clone());
			g.insert(end_a.clone(), s);
		}
		if let Some(ref mut s) = g.get_mut(&end_b) {
			s.insert(end_a.clone());
		} else {
			let mut s = BTreeSet::new();
			s.insert(end_a.clone());
			g.insert(end_b.clone(), s);
		}
	}
	let mut q = VecDeque::<(&String, BTreeSet<&String>, bool)>::new();
	let start_string = "start".to_string();
	q.push_back((&start_string, BTreeSet::<&String>::from([&start_string]), false));
	let mut res = 0u64;
	while let Some((node, v, small_visited_twice)) = q.pop_front() {
		if node == "end" {
			res += 1;
		} else {
			for n in g.get(node).unwrap().iter() {
				if n.as_bytes()[0] < b'a' {
					q.push_back((n, v.clone(), small_visited_twice));
				} else if n != "start" && (!v.contains(&n) || !small_visited_twice) {
					let mut vv = v.clone();
					vv.insert(&n);
					q.push_back((n, vv, small_visited_twice || v.contains(&n)));
				}
			}
		}
	}
	println!("{}", res);
}
