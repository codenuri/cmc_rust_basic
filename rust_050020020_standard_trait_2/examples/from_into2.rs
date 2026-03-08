#[derive(Copy, Clone, Debug)]
struct Point {
	x : i32,
	y : i32,
}

impl From<(i32, i32)> for Point {

	fn from(value:(i32, i32))-> Self {
		Self{ x:value.0, y:value.1 }
	}
}

fn main() {

	let pt = Point::from((1, 2));	 

	println!("{:?}", pt);
}