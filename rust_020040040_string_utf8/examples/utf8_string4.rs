fn main() {

	let mut s = String::from("A가BC");

	println!("{}",   s.chars().count()); // 4
	println!("{:?}", s.chars().nth(2));	 // Some('B')

	for c in s.chars().rev().step_by(2) {
		print!("{c}, ")	// C, 가
	}
	println!();
}
