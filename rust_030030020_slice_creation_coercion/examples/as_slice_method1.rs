fn main() {

	let x = [0, 1, 2, 3, 4];

	let s1 = &x[1..4];
	let s2 = x.as_slice();

	println!("{s1:?}");  // [1, 2, 3]
	println!("{s2:?}");  // [0, 1, 2, 3, 4]
}

