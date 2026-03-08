fn main() {

	let s = String::from("ABCD");
	
	println!("{:p}", s.as_ptr());
	println!("{}",   s.len());
	println!("{}",   s.capacity());

	println!("{}", std::mem::size_of_val(&s)); // 24
}
