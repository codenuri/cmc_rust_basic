fn main() {

	let n : i32 = 10;

	let r : &i32       = &n;
	let p : *const i32 = &n;	

	let a = *r;				// ok
//	let b = *p;				// error
	let b = unsafe {*p}; 	// ok

	println!("{a}, {b}");
}
