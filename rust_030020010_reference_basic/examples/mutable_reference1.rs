fn main() {

	let mut n1 = 10;
	let mut n2 = 10;

	let r1 : &i32     = &n1;
	let r2 : &mut i32 = &mut n2;

	*r1 = 20; // error
	*r2 = 20; // ok

	//-------------------------------------
	let n1 = 10;
	let mut n2 = 10;

	let r1 : &i32     = &n1;		// ok
//	let r2 : &mut i32 = &mut n1;	// error	

	let r3 : &i32     = &n2;		// ok
	let r4 : &mut i32 = &mut n2;	// ok	
}
