fn main() {
	
	let v1 : i32  = 0;		// ok. i32 type variable
	let r1 : &i32 = &v1;	// ok. i32 type reference

	let v2 : [i32;5] = [0;5];// ok. array type variable
	let r2 : &[i32;5] = &v2; // ok. array type reference

	let v3 : [i32];		// error.
	let r3 : &[i32];	// ok. slice reference
}
