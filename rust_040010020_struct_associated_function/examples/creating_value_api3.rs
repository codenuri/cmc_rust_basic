#[derive(Default)]
struct Point { 
	x : i32, 
	y : i32, 
}

impl Point {
	fn new(x:i32, y:i32 ) -> Self { 
		Self{x, y}	
	}
}

fn main()
{
	let _pt = Point{x:1, y:1};
	let _pt = Point::new(1, 1);
    let _pt = Point::default();    
}
