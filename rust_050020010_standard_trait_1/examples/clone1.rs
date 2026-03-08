struct Point {
	x : i32,
	y : i32,
}

impl Clone for Point {
	fn clone(&self) -> Self {
		Self{x:self.x, y:self.y}
	}
}

fn main() {
	let pt1 = Point{x : 1, y : 1};
	let pt2 = Point{x : 2, y : 2};

	let mut pt3 = pt1.clone();
	println!("{}, {}", pt3.x, pt3.y);	// 1, 1

	pt3.clone_from(&pt2); // pt3 = pt2.clone()
	println!("{}, {}", pt3.x, pt3.y);	// 2, 2
}
