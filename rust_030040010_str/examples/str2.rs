fn main() {

	println!("BBB");

	let sa1 = String::from("AAA");
	let sa2 = String::from("AAA");

	let sb1 = sa1.as_str();
	let sb2 = "AAA";
	let sb3 = "AAA";
	
	sa1.push('B');	// ok
	sb1.push('B'); 	// error
}