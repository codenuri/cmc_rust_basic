fn main() {

	let s = 75;

	let g = if s > 60 { "PASS" } 
			//	 else { "FAIL" }; 	// ok
			//	 else { 0 }; 	  	// error
				 else { panic!() }; // ok
			//	 else { return }; 	// ok
			//	 else { std::process::exit(-1) }; // ok
			//	 else { loop {} }; 		 // ok
			//	 else { while true {} }; // error

	loop {

		let g = if s > 60 { "P" } else { break }; 	// ok
		let g = if s > 60 { "P" } else { continue };// ok

		break;
	}
}