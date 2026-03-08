trait MyTrait {

	fn m1(&self);		// required method
	fn m2(&self) { }	// provided method

	fn f1();		// required associated function
	fn f2() { }		// provided associated function

	type Output1;			// associated type
//	type Output2 = i32;		// error. unstable

	const SIZE1:usize;		// associated constant
	const SIZE2:usize = 32;	// ok
}

fn main() { 

}