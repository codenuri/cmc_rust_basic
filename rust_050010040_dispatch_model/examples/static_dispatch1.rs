trait Capture {	fn take(&self); }

struct Camera;
struct HDCamera;

impl Capture for Camera {
	fn take(&self) { println!("Camera take");}
}

impl Capture for HDCamera {
	fn take(&self) { println!("Camera take");}
}

fn use1(c : &Camera ) { 
	c.take();	// ok
}

fn use2<T>(c : &T ) { 
	c.take();	// error. 
}

fn use3<T:Capture>(c : &T ) { 
	c.take();	// ok 
}

fn use4(c :&impl Capture) { 
	c.take();	// ok 
}







fn main() {	
	
	let c1 = Camera{};
	let c2 = HDCamera{};

	use1(&c1);	// ok
	use1(&c2);	// error
	use3(&c1);	// ok
	use3(&c2);	// ok
	use4(&c1);	// ok
	use4(&c2);	// ok
}

