fn main() {

	let v = vec![1,2,3,4,5];

	
	for e in v { 		// it = v.into_iter()
		
		println!("{e}");

	}

	println!("{v:?}"); // error
}
