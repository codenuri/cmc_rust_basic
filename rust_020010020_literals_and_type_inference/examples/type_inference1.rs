#![allow(unused)]

fn main() {
	let num = 10;

	let n1 : i32 = 10;		
	let n2       = 10i32;	

	let _ : u8 = n1;	// error
	let _ : u8 = n2;	// error
	//--------------------------------------

	let n3 = 10;
	let _ : u8 = n3;	// ok
	let _ : i32 = n3;	// error

	let n4 = 10;
	let _ : f32 = n4;	// error
}

