struct Point {
	x : i32,
	y : i32, 
}

enum Color {
	Red, Green, Blue
}

fn main() {
	
	let p : Point = Point{x:1, y:2};
	let c : Color = Color::Red;

	let x = p.x;	
}
