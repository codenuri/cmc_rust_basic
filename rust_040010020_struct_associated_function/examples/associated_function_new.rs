#![allow(unused)]

#[derive(Copy, Clone, Debug)]
struct Point { 
	x : i32, 
	y : i32, 
}

impl Point {

	fn new(x : i32, y : i32) -> Self {		
		Self{x, y}
	}

}


fn main() {

	let s1 = String::new();
	let v1 = Vec::<i32>::new();

	let p1 = Point{x:1, y:1};
	let p2 = Point::new(1, 1); // ?	

	println!("{p2:?}");
}

struct Point { 
	x : i32, 
	y : i32, 
}

impl Point {

	// 1. 함수
	//    associated function
	//    method

	// 2. associated type

	// 3. associated constant
}