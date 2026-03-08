fn main() {
	
	let mut s = String::from("ABCD");

	println!("{}, {}", s.len(), s.capacity() ); // 4, 4

	s.push('E'); 	// capacity == len

	println!("{}, {}", s.len(), s.capacity() );	// 5, 8
					
	s.push('F');	// capacity > len

	println!("{}, {}", s.len(), s.capacity() ); // 6, 8

}

