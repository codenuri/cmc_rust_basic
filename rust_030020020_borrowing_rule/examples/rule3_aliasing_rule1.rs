fn main() {

	let mut n = 10;

	let r  = &mut n;	// start borrowing

	let _r = &n; 		// error
	let _r = &mut n; 	// error
	let _v = n;			// error

	*r = 30;			// end borrowing
}