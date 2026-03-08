fn main() {

	let mut s = String::from("A가BC");


	let bs = s.as_bytes(); // String => slice(&[u8])

	println!("{bs:?}"); // [65, 234, 176, 128, 66, 67]
					 

	// #1. byte 길이와 char 갯수
	println!("{}",   s.len());			// byte 길이
	println!("{}",   s.chars().count());// char 개수


	// #2. s[index] 연산을 사용할수 없다.
//	let c = s[2];	// error
}
