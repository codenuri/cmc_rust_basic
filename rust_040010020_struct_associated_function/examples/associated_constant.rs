#[derive(Debug, Default)]
struct Point { 
	x : i32, 
	y : i32, 
}
impl Point {

	const ORIGIN : Self  = Self {x:0, y:0};	

	fn new(x:i32, y:i32 ) -> Self { 
		Point{x, y}	
	}
}
fn main()
{
	// (0, 0) 의 Point 변수를 만드는 다양한 방법
	let p1 = Point{x:0, y:0};
	let p2 = Point::new(0, 0);
	let p3 = Point::default();	
	let p4 = Point::ORIGIN;

	println!("{p1:?}");
	println!("{p2:?}");
	println!("{p3:?}");
	println!("{p4:?}");
}
