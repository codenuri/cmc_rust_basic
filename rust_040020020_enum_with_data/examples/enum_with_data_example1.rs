#![allow(unused)]

struct Rect {
	left  : i32, top   : i32,
	right : i32, bottom: i32,
}
impl Rect {
	fn draw(&self) { println!("Draw Rect");}
}
struct Circle {
	x : i32, y : i32, radius : i32,
}
impl Circle {
	fn draw(&self) { println!("Draw Circle");}
}

fn main() {
	
	let mut v = Vec::<Rect>::new();

}