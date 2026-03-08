fn main() {

	let mut v = vec![1, 2, 3, 4, 5];

	let mut it = v.iter();
	
	while let Some(e) = it.next() {

		*e = 0; 	// error. 
					// e 는 &mut i32 가 아닌 &i32
	}

	let mut it = v.iter_mut();

	while let Some(e) = it.next() {
				
		*e = 0; 		 // ok. e 는 &mut i32
	}	
	println!("{v:?}");
}