fn main() {

	let s1 = String::from("ABCD");

	if false {

		let s2 = s1; // move

	}

	println!("{s1}"); // error
}
