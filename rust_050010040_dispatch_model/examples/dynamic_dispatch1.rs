trait Capture {	fn take(&self); }

struct Camera;
struct HDCamera;

impl Capture for Camera {
	fn take(&self) { println!("Camera take");}
}

impl Capture for HDCamera {
	fn take(&self) { println!("Camera take");}
}

fn main() {	
	
	let c1 = Camera{};
	let c2 = HDCamera{};

	let r : &Camera = &c1; // ok
//	let r : &Camera = &c2; // error

//	let c : Capture; // error	

	let r : &dyn Capture = &c1; // ok
	let r : &dyn Capture = &c2; // ok
}
