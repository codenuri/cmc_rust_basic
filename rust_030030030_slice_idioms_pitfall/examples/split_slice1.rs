fn main() {

	let mut a = [9, 8, 7, 6, 5, 4, 3, 2, 1];

	let s1 = &mut a[0..4];
	let s2 = &mut a[4..8];

	s1.sort();

	println!("{a:?}");
}

