fn main() {

	let mut v = vec![1, 2, 3, 4, 5];

	
//	let ret = v[7]; 		// panic  발생
	let ret = v.get(7); 	// Option 으로 반환
	println!("{ret:?}"); 	// None



	match v.get(3) {
//		Some(v)  => { // v : &i32
		Some(&v) => { // v : i32
			println!("{}", std::any::type_name_of_val(&v));
		},
		None => {},
	}


	match v.get_mut(3) {
		Some(v)      => { 	// v : &mut i32
//		Some(&mut v) => { 	// v : i32
//		Some(&v)     => { 	// error		
			println!("{}", std::any::type_name_of_val(&v));

			*v = 100;	// v[3] = 100;
		},
		None => {},		
	}

	println!("{}", v[3]); // 100
}