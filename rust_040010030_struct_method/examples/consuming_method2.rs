fn main() {

	let s1 : String = String::from("ABCD");
	let s2 : String = String::from("ABCD");

	let v1 : Vec<u8> = s1.into_bytes();
	let v2 : Vec<u8> = s2.clone().into_bytes();

	println!("{v1:?}"); // [65, 66, 67, 68]

//	println!("{}", s1); // error
	println!("{}", s2); // ok


	// parse() 메소드는 Result<> 타입 반환
	let ret = "10".parse::<i32>(); // ret : Ok(10)
				 
	let n = ret.unwrap(); // unwrap(self) 

	println!("{ret:?}"); // error		
}