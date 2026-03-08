#![allow(unused)]

#[derive(Copy, Clone, Debug)]
struct Point {
	x : i32,
	y : i32,
}

impl From<(i32, i32)> for Point {
	fn from(value:(i32, i32))-> Self {
		Point{ x:value.0, y:value.1 }
	}
}

fn main() {

	let n1 : u16 = 10;
//	let n2 : u32 = n1; // error
	let n3 : u32 = n1.into(); // ok

	let pt : Point = (1, 2).into(); // ?	
					// Point::from((1,2))
	println!("{pt:?}");
}