fn main() {

	let v = vec![1, 2, 3, 4, 5];

	let s : i32 = v.iter().sum();
	let mut it  = v.iter().rev();

	for e in it {
		print!("{e}, "); 	// 5, 4, 3, 2, 1
	}
	println!();

	for e in v.iter().take(4) {
		print!("{e}, ");	// 1, 2, 3, 4
	}
	println!();

	for e in v.iter().filter(|e| *e % 2 == 0 ) {
		print!("{e}, ");	// 2, 4
	}
	println!();
}
