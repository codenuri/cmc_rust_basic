fn main() {

	let mut s1 = String::from("abcd");

	let _s2 = s1; 	  // move

	println!("{s1}"); // error
			// ^^ value borrowed here after move

	s1 = String::from("hello");	// 다시 자원 획득

	println!("{s1}");	// ok
}
