trait Capture {
	fn take(&self);
}

struct Camera;

impl Camera {

	fn flash(&self) { 
		println!("flash");
	}
}

impl Capture for Camera {	

	fn take(&self) { 
		println!("take picture");
	}
}

fn use_camera( c : &impl Capture ) {
	c.take();
}
fn use_camera( c : &dyn  Capture ) { 
	c.take();
}
fn use_camera<T:Capture>( c : &T ) { 
	c.take();
}

fn main() {

}
