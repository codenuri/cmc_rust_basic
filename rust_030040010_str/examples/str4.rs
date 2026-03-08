fn main() {

	let s1 = "source_code.txt";				// s1 : &str
	let s2 = "source_code.txt".to_string(); // s2 : String

	println!("{}", s1.ends_with(".txt"));
	println!("{}", s2.ends_with(".txt"));	

	println!("{:?}", s1.find("code"));
	println!("{:?}", s2.find("code"));	

}

