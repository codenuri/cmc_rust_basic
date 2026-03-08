fn main() {

	let mut s = String::from("ABCDEFG");
    
    let bytes = unsafe { s.as_bytes_mut() };

	println!("{}", std::any::type_name_of_val(&bytes));
											// &mut [u8]

	let c = bytes[1];

	bytes[1] = b'Z';
	
	println!("{}", c); // 66
    println!("{}", s); // AZCD	
}