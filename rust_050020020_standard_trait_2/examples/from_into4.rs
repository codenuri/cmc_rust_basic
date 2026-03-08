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

// (i32, i32) 에 대한 From<Point> 제공
// From<Point> 에서 Point 때문에 orphan rule 적용되지 않음
impl From<Point> for (i32, i32) {
	fn from(p:Point)-> Self {
		(p.x, p.y)
	}
}

fn main() {

	let pt = Point::from((1,2));
	let tp = <(i32, i32)>::from(pt);
	
	let pt : Point      = (1, 2).into();
	let tp : (i32, i32) = pt.into();
}