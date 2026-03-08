struct Point {
	x:i32,
	y:i32,
}
fn main() {

	let n = 10;
	
	let r : &i32 = &n;
	let r        = &n;

	let ref r : &i32 = n;
	let ref r        = n;

	let &v = r;	// r : &i32

	// reference to literal
	let r = &10;
	let r = &Point{x:1, y:1}
	let b = (0..10).contains(&3);	
}

