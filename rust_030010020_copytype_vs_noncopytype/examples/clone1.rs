fn main() {

	let s1 = String::from("ABCD");
	let s2 = String::from("ABCD");

	let s3 = s1;			// move 
	let s4 = s2.clone();	// deep copy

	println!("{s1}"); 	// error
	println!("{s2}"); 	// ok

}

