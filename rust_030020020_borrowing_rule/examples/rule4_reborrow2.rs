fn main() {

	let mut n = 10;

	let r  = &mut n;	// start borrowing
	let rr = &*r;   	// start immutable reborrowing

	let _v = *r;		// ok     r 로 읽기 가능
//	*r = 30;			// error  r 로 쓰기 안됨	

	println!("{}", *rr);// end immutable reborrowing
	*r  = 30;			// end borrowing
}