trait Capture {	fn take(&self); }

fn main() {		

	let n1 : dyn Trait;  // error, 
	let r1 : &dyn Trait; // ok

	let n2 : [i32];		// error
	let r2 : &[i32];	// ok

	let n3 : str;		// error
	let r3 : &str;		// ok
}

