fn main() {

//	let mut x1 = [0, 1, 2, 3, 4];
//	let mut x2 = [0, 1, 2, 3, 4];
//	let mut x3 = [0, 1, 2, 3, 4];
//	let mut x4 = [0, 1, 2, 3, 4];

	let mut x1 = vec![0, 1, 2, 3, 4];
	let mut x2 = vec![0, 1, 2, 3, 4];
	let mut x3 = vec![0, 1, 2, 3, 4];
	let mut x4 = vec![0, 1, 2, 3, 4];

	let s1 = x1.as_slice();
	let s2 = x2.as_mut_slice();
	let s3 : &[i32]     = x3.as_ref();
	let s4 : &mut [i32] = x4.as_mut();

	println!("{s1:?}");
	println!("{s2:?}");
	println!("{s3:?}");
	println!("{s4:?}");
}

