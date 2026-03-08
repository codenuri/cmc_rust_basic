#![allow(unused)]

fn main()
{
	let c1 : char = 'A';	
	let c2 : u8   = b'A';

	println!("{}", std::mem::size_of_val(&c1) ); // 4
	println!("{}", std::mem::size_of_val(&c2) ); // 1

	let s1 : &str  = "hello";
	let s2 : &[u8] = b"hello";	
}
