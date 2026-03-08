trait Capture {
	fn take(&self);
}


struct Camera;

//impl Camera {
impl Capture for Camera {	
	fn take(&self) { println!("take picture");}
}

struct HDCamera;

//impl HDCamera {
impl Capture for HDCamera {		
	fn take(&self) { println!("take HD picture");}
}

fn use_camera( c : &impl Capture ) { c.take(); }

fn main() {

	let c1 = Camera{};
	use_camera(&c1);

	let c2 = HDCamera{};
	use_camera(&c2);	// ok
}
