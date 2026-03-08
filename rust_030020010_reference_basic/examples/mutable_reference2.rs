fn main() {

	let mut n1 = 10;
	let mut n2 = 10;
	let mut n3 = 10;

	let mut r1 : &i32 = &n1;	
	let     r2 : &mut i32 = &mut n3;

	r1 = &n2;	// ok
	*r1 = 20;	// error

	r2 = &mut n2; // error
	*r2 = 20;     // ok

	let mut r3 : &mut i32 = &mut n3;
}