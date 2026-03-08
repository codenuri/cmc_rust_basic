fn main() {

	let s = String::from("A가BC");

//	let c = s[0];	// error
//	for c in s 	{	// error
//	}

	for c in s.bytes() 	{		
		print!("{c}, ");	// 65, 234, 176, 128, 66, 67,
	}
	println!();

	for c in s.chars() 	{		
		print!("{c}, ");	// A, 가, B, C,
	}
	println!();

	for c in s.char_indices() 	{		
		print!("{c:?}, ");	// (0, 'A'), (1, '가'), 
	}						// (4, 'B'), (5, 'C'),
	println!();
}
