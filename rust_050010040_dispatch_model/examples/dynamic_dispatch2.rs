trait Capture {	fn take(&self); }

struct Camera;
struct HDCamera;

impl Capture for Camera {
	fn take(&self) { println!("Camera take");}
}

impl Capture for HDCamera {
	fn take(&self) { println!("Camera take");}
}

fn use1(c : &dyn Capture ) {

	c.take();
}

fn main() {	
	
	let c1 = Camera{};
	let c2 = HDCamera{};	

	use1(&c1);
	use1(&c2);
}
