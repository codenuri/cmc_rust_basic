fn main() {
	
	let mut x1 = [0, 1, 2, 3, 4];
	let mut x2 = [0, 1, 2, 3, 4];
	let mut x3 = [0, 1, 2, 3, 4];
	let mut x4 = [0, 1, 2, 3, 4];

	let     s1 : &[i32]     = &x1[1..4];
	let     s2 : &mut [i32] = &mut x2[1..4];
	let mut s3 : &[i32]     = &x3[1..4];
	let mut s4 : &mut [i32] = &mut x4[1..4];

	s1[0] = 10;	// error
	s2[0] = 10;	// ok
	s3[0] = 10;	// error
	s4[0] = 10;	// ok
	
	s1 = &x1[0..2]; 	// error
	s2 = &mut x2[0..2];	// error
	s3 = &x3[0..2];		// ok
	s4 = &mut x4[0..2];	// ok	
}
