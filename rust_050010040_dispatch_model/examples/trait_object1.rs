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
	let user_input = 1;

	let c1 = Camera{};
	let c2 = HDCamera{};
	let c3 = HDCamera{};

	let mut r : &dyn Capture;

	if user_input == 1 {
		r = &c1;
	}
	else {
		r = &c2;
	}
	r.take();

	println!("{}", std::mem::size_of_val(&r));	// 16

	println!("{r:p}");	// variable addr, vtable addr	
}

