fn main() {

	let mut n = 10;

	let r  = &mut n;	// start borrowing
//	let rr = &mut n;    // error
	let rr = &mut *r;   // start mutable reborrowing

	let _v = *r;		// error

	*rr = 20;			// end mutable reborrowing
	*r  = 30;			// end borrowing
}