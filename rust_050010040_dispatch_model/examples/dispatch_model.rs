trait Capture {	fn take(&self); }

struct Camera;
struct HDCamera;

impl Capture for Camera {
	fn take(&self) { println!("Camera take");}
}

impl Capture for HDCamera {
	fn take(&self) { println!("Camera take");}
}

fn use1(c : &Camera )       { c.take();}
fn use2(c : &impl Capture ) { c.take();}
fn use3(c : &dyn  Capture ) { c.take();}

fn main() {	
	
	let c1 = Camera{};
	let c2 = HDCamera{};

	use1( &c1 );	// ok
	use1( &c2 );	// error
	use2( &c1 );	// ok
	use2( &c2 );	// ok
	use3( &c1 );	// ok
	use3( &c2 );	// ok		
}

