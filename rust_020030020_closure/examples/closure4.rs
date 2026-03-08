fn main() {

	let f1 = |a:i32, b:i32| -> i32 {a + b};

	println!("{}", f1(1, 2) ); // 3


	let f2 = |a, b| a + b;

	println!("{}", f2(1, 2) );		// {integer} 
//	println!("{}", f2(1.1, 2.2) ); 	// error
	println!("{}", f2(1u8, 2) );	// u8 로 결정
}
