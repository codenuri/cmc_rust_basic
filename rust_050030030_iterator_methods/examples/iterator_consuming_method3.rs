fn main() {

	let mut v = vec![1, 2, 3];

	v.iter().for_each( |e| print!("{e}, ") ); // 1, 2, 3

	println!();

	v.iter().     for_each( |e| println!("{}", std::any::type_name_of_val(&e)) );	// e : &i32
	v.iter_mut(). for_each( |e| println!("{}", std::any::type_name_of_val(&e)) );	// e : &mut i32
	v.into_iter().for_each( |e| println!("{}", std::any::type_name_of_val(&e)) );	// e : i32

	let mut v1 = vec![1, 2, 3];
	let mut v2 = vec![1, 2, 3];
	
	v1.into_iter().for_each( |e| { println!("{}", std::any::type_name_of_val(&e)) } );		// i32
	v2.into_iter().find(     |e| { println!("{}", std::any::type_name_of_val(&e)); true} ); // &i32

}

