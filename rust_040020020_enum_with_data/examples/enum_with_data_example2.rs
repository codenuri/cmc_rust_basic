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
//----------------------------------
enum Shape {
	Rect(Rect),
	Circle(Circle),
}
impl Shape {
	fn draw(&self) {
		match self {
			Shape::Rect(r)   => r.draw(), 
			Shape::Circle(c) => c.draw(), 
		}
	}
}

fn main() {
	
	let mut v = Vec::<Shape>::new();

	v.push( Shape::Rect(Rect{left:1, top:1, right:10, bottom:10}));
	v.push( Shape::Circle(Circle{x:1, y:1, radius:10}));
	v.push( Shape::Rect(Rect{left:1, top:1, right:10, bottom:10}));
	v.push( Shape::Circle(Circle{x:1, y:1, radius:10}));

	for s in v {
		s.draw();
	}
}