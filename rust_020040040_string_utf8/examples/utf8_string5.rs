fn main() {

	let mut s = String::from("A가BC");

	s.replace_range(1..4, "D");
	
	println!("{s}");		// "ADBC"


	s = String::from("A가BC");
	let n = 1;
	let start = s.char_indices().nth(n);	// Some((1, '가'))
	let end   = s.char_indices().nth(n+1);	// Some((4, 'B'))

	println!("{start:?}, {end:?}");
}