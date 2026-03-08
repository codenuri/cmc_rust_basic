struct Point1 {
	x : i32,
	y : i32,
}
impl Default for Point1 {
	fn default() -> Self {
		Self{x:0, y:0}
	}
}

#[derive(Default)]
struct Point2 {
	x : i32,
	y : i32,
}

fn main() {
	let pt1 = Point1::default();
	let pt2 = Point2::default();

	println!("{}, {}", pt1.x, pt1.y); // 0, 0
	println!("{}, {}", pt2.x, pt2.y); // 0, 0
}
