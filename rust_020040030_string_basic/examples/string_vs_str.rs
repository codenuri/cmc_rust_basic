fn main() {

	println!("AAA");
	println!("BBB");

	let s1 : &str   = "AAA";
	let s2 : &str   = "AAA";
	let s3 : String = String::from("AAA");
	let s4 : String = String::from("AAA");
	let s5 : &str   = &s4;
	
	println!("{:p}", s1);			// 주소 A
	println!("{:p}", s2);			// 주소 A
	println!("{:p}", s3.as_ptr()); 	// 주소 B
	println!("{:p}", s4.as_ptr()); 	// 주소 C
	println!("{:p}", s5);		   	// 주소 C
}