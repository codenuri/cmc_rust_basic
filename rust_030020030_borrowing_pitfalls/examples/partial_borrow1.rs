fn main() {

	let mut t = (10, 3.4);

	let r = &mut t;		// <= 대여 시작, whole borrow

//	let r1 = &t;		// error
//	let r2 = &t.0;		// error
//	let r3 = &t.1;		// error

	println!("{:?}", *r);	// <= 대여 반납
	//---------------------------------
	let r = &mut t.0;	// <= 대여 시작, partial borrow

//	let r1 = &t;		// error
//	let r2 = &t.0;		// error
	let r3 = &t.1;		// ok

	println!("{:?}", *r);	// <= 대여 반납	
}
