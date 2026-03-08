#[derive(Debug)]
struct Point1 {
	x : i32,
	y : i32,
}

struct Point2 {
	x : i32,
	y : i32,
}

impl Debug for Point2 {
}

fn main() {
	let pt1 = Point1{x : 1, y : 2};
	let pt2 = Point2{x : 1, y : 2};

	println!("{pt1:?}");
	println!("{pt2:?}");
	
}
