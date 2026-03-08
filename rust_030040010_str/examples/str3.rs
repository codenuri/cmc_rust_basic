fn main() {

	let s1 = "ABC";					// s1 : &str	
	let s2 = "ABC".to_string();		// s2 : String
	let s3 = s1.to_string();		// s3 : String
	let s3 = String::from("ABC");	// s3 : String

	// ------------------------------------------
	let s = String::from("A가BC");

	let s1 = s.as_str();// s1 : &str	
	let s2 : &str = &s; // &String => &str
	let s3        = &s; // s3 : &String

	let s4 = &s[0..4]; // "A가"
//	let s5 = &s[0..3]; // panic!()
}
