fn main() {

	let a = [0, 1, 2, 3, 4];

	let s1 = &a[1..4];
	let s2 = &a[1..=3];
	let s3 = &a[1..];
	let s4 = &a[..3];
	let s5 = &a[..=3];
	let s6 = &a[..];
	let s7 = &s5[1..];

	println!("{s1:?}");
	println!("{s2:?}");
	println!("{s3:?}");
	println!("{s4:?}");
	println!("{s5:?}");
	println!("{s6:?}");
	println!("{s7:?}");

	let mut a1 = [0, 1, 2, 3, 4];
	let mut a2 = [0, 1, 2, 3, 4];
	
	let s1 = &a1[1..4];
	let s2 = &mut a2[1..4];

//	s1[0] = 3; // error
	s2[0] = 3; // ok 	
}
