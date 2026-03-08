fn main() {

	let mut v = vec![1, 2, 3];

//	let ret = v[5]; // panic!()
	let ret = v.get(5); // None

	println!("{}", std::any::type_name_of_val(&ret));
						 // Option<&i32>

	if let Some(e) = v.get(1) {

		println!("{e}"); // 2
		
		let n1 = e;		// n1 : &i32
		let n2 = *e;	// n2 : i32
	
		println!("{}", std::any::type_name_of_val(&n1));
		println!("{}", std::any::type_name_of_val(&n2));
	}
}