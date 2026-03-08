fn main() {

	let v = vec!["A".to_string(), "B".to_string()];

	let mut it = v.into_iter();

//	println!("{v:?}");	// error. 
						// value borrowed here after move

	while let Some(e) = it.next() {	
		
		// e 는 reference 가 아닌 value
		println!("{}, {e}", std::any::type_name_of_val(&e));		 		 
	}	


}
