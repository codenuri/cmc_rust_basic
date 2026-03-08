fn main() {

	let n1 = 10;	
	let s1 = String::from("ABCD");

	let n2 = n1;	// n1 copy type
	let s2 = s1;	// s1 non-copy type

	println!("{n1}"); // ok
	println!("{s1}"); // error
}
