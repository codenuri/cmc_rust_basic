#[derive(Debug)]
struct Point {
	x : i32,
	y : i32,
}

fn main() {
	let pt = Point{x:1, y:2};

//	println!("{pt}");   // error
	println!("{pt:?}"); // ok  Point { x: 1, y: 2 }

}

