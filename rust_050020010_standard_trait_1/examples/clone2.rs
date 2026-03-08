#[derive(Clone)]
struct Point {
	x : i32,
	y : i32,
}


fn main() {
	let pt1 = Point{x : 1, y : 1};
	let pt2 = Point{x : 2, y : 2};

	let pt3 = pt1; 
	let pt4 = pt2.clone();

	println!("{}", pt1.x); 	// error. 
							// value borrowed here after move
	println!("{}", pt2.x);	// ok
}
