#[derive(Copy, Clone)]
struct Point  {
	x : i32,
	y : i32,
}

fn main() {
	
	let p1 = Point{x:1, y:2};

	let p2 = p1; // ?
	let p3 = p1.clone();
	
	println!("{}", p1.x); // ok
}

