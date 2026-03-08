fn main() {

	let v = vec![1, 2, 3, 4, 5];

	let mut it = v.iter();

	while let Some(e) = it.next() {
		
		print!("{e}, ");
	}
	println!();
}

