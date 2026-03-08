fn main() {

	let a = [0, 1, 2, 3, 4];

	let mut r = &a;
	let mut s = a.as_slice();

	println!("{r:?}");  // [0, 1, 2, 3, 4]
	println!("{s:?}");  // [0, 1, 2, 3, 4]

	println!("{}", std::any::type_name_of_val(&r));// &[i32;5]
	println!("{}", std::any::type_name_of_val(&s));// &[i32]	

	r = &x[0..3]; // error
	s = &x[0..3]; // ok

}



