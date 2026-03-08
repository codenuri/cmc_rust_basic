fn main() {

	let s1 = String::from("abcd");

	println!("{s1}"); // ok, "abcd"

	let s2 = s1; 	  // 이 문장 이후 s1 사용시 에러

	println!("{s1}"); // error
			// ^^ value borrowed here after move
}
