#![allow(unused)]

fn main() {

	let mut n1 : i32;

//	println!("{n1}"); // error
	n1 = 10;		  // ok
	println!("{n1}"); // ok

	let n2 : i32;	// n2 는 immutable
	n2 = 10; 		// ok
//	n2 = 20; 		// error
	
	let n3;
	n3 = 30;
}
