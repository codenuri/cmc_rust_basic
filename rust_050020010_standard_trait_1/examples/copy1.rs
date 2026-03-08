struct Point {
	x : i32,
	y : i32,
}

impl Clone for Point {
	fn clone(&self) -> Self {
		Self{x:self.x, y:self.y}
	}
}

impl Copy for Point {}

fn main() {

	let pt1 = Point{x : 1, y : 1};
	let pt2 = pt1;

	println!("{}, {}", pt1.x, pt1.y);
	println!("{}, {}", pt2.x, pt2.y);
}
