fn main() {
	
	let mut a = [1, 3, 5, 7, 9, 2, 4, 6, 8, 10];
	let mut v = vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 10];

	// a.sort();
	// a.reverse();

	// v.sort();
	// v.reverse();

	a[2..7].sort();
	a[2..7].reverse();
	v[2..7].sort();
	v[2..7].reverse();
	
	println!("{a:?}");
	println!("{v:?}");
}
