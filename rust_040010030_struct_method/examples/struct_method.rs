struct Point { 
	x : i32, 
	y : i32, 
}

impl Point {
	fn new(x:i32, y:i32) -> Self { Self{x, y} }

	fn get_x(&self) -> i32 { self.x }	

	fn set(&mut self, x : i32, y : i32) { 
		self.x = x;
		self.y = y;
	}	
}

fn main()
{
	let mut pt = Point::new(1, 1);

	pt.set(10, 20);		

	let x = pt.get_x();

	println!("{x}"); // 10

	let     p1 = Point::new(1, 1);
	let mut p2 = Point::new(1, 1);

	p1.get_x();	// ok
	p2.get_x();	// ok

//	p1.set(10, 20);	// error
	p2.set(10, 20);	// ok	
}
