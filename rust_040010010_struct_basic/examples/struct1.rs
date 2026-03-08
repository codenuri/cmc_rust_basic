struct Point {
	x : i32,
	y : i32,	
}

fn main() {

	let pt : Point = Point{x:3, y:3};
	let pt         = Point{x:3, y:3};
	let mut pt     = Point{x:3, y:3};

	println!("{}, {}", pt.x, pt.y);
}
	