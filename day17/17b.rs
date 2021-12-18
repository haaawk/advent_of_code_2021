fn main() {
	let (xb, xe) = (150, 193);
	let (yb, ye) = (-86, -136);

	let mut count = 0;
	for x in 0..1000 {
		for y in -1000..1000 {
			let mut vx = x;
			let mut vy = y;
			let mut px = 0;
			let mut py = 0;
			loop {
				px += vx;
				py += vy;
				if px > xe || py < ye {
					break;
				}
				if vx != 0 {
					vx -= 1;
				}
				vy -= 1;
				if px >= xb && py <= yb {
					println!("{}, {}", x, y);
					count += 1;
					break;
				}
			}
		}
	}
	println!("{}", count);
}
