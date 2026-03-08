#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
	x : i32,
	y : i32,	
}

fn main() {

	let p1 = Point{x:3, y:3};
	let p2 = Point{x:3, y:3};

	println!("{:?}", p1);	// ?

	println!("{}", p1 == p2 ); // ?
	println!("{}", p1 != p2 ); // ?
}
