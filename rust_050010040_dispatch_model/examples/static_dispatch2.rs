trait Capture {	fn take(&self); }

#[derive(Copy, Clone)]
struct Camera;
#[derive(Copy, Clone)]
struct HDCamera;

impl Capture for Camera {
	fn take(&self) { println!("Camera take");}
}

impl Capture for HDCamera {
	fn take(&self) { println!("Camera take");}
}

fn use1(c : impl Capture ) { 
	c.take();	
}
fn use2(c : &impl Capture ) { 
	c.take();	
}
fn use3(c : &mut impl Capture ) { 
	c.take();	
}
fn main() {	
	
	let mut c1 = Camera{};
	let mut c2 = HDCamera{};

	use1(c1);	// ok
	use1(c2);	// ok
	use2(&c1);	// ok
	use2(&c2);	// ok	
	use3(&mut c1);	// ok
	use3(&mut c2);	// ok	
}

