struct Camera;

impl Camera {
	fn take(&self) { println!("take picture");}
}

struct HDCamera;

impl HDCamera {
	fn take(&self) { println!("take HD picture");}
}

fn use_camera( c : &Camera ) { c.take(); }

fn main() {

	let c1 = Camera{};
	use_camera(&c1);

	let c2 = HDCamera{};
	use_camera(&c2);	// error
}
