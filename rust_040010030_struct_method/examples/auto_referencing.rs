struct Point { 
	x : i32, 
	y : i32, 
}

impl Point {
	fn set(&mut self, x : i32, y : i32) { 
		self.x = x;
		self.y = y;
	}	
}

fn main()
{
	let mut pt = Point::new(1, 1);

	pt.set(10, 20);		
	
	Point::set(&mut pt, 10, 20);
}
