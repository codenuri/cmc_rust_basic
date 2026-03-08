#![allow(unused)]

struct MyType { 
	copyfield : i32,
	movefield : String, 
}

fn main() {
	
	let m1 = MyType{copyfield:1, 
					movefield:String::from("1")};

	let m2 = m1; 			// moved whole value
//	let v  = m1;			// error
//	let v  = m1.copyfield;	// error
//	let v  = m1.movefield;	// error

//	let m2 = MyType{..m1}; 	// field-wise move/copy
							// m2.copyfield = m1.copyfield
							// m2.movefield = m1.movefield
//	let v  = m1;		   	// error
	let v  = m1.copyfield;	// ok
//	let v  = m1.movefield;	// error
}
