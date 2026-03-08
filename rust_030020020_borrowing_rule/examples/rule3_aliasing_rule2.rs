fn main() {

	let mut n = 10;

	let r1  = &n;	// start borrowing1
	let r2  = &n;	// start borrowing2

	let _v = *r1;	// ok read
	let _v = *r2;	// ok read
	let _v = n;		// ok read
	n = 20;			// error write

	println!("{}", *r2);	// end borrowing2
	println!("{}", *r1);	// end borrowing1
}