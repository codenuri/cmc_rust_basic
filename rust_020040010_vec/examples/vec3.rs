fn main() {

	let v = vec![1i32, 2, 3];

	// #1. panic!
//	let _n = v[5];	// panic!()

	// #2. get() method
	println!("{:?}", v.get(5));	// None
	println!("{:?}", v.get(0));	// Some(1)

	if let Some(n) = v.get(0) {
		println!("{n}"); 	// 1
	}


	// #3. Option<&i32>
	let ret = v.get(0);
	println!("{:?}", std::any::type_name_of_val(&ret));	
								// Option<&i32>

	if let Some(n) = v.get(0) {
		println!("{n}"); 
		println!("{}", std::any::type_name_of_val(&n));

		let k1 = n; 	// k1 : &i32
		let k2 = *n; 	// k2 : i32
	}	

	// #5. &n 패턴
	if let Some(&n) = v.get(0) {
		println!("{}", std::any::type_name_of_val(&n));
								// i32
	}

	// #6. get(), get_mut()
	let mut v = [1, 2, 3];

	if let Some(n) = v.get(0) {		// Option<&i32>
	//	*n = 10; 	// error
	}
	if let Some(n) = v.get_mut(0) {	// Option<&mut i32>
		*n = 20;	// ok
	}
	println!("{v:?}")
}


enum Option<T> {
	Some(T),
	None,
}