fn foo() -> Result<i32, i32> {

//	Result::Ok(300)
//	Result::Err(100)
	Ok(300)
}

fn main() {

	let ret = foo();

	match ret {		
//		Result::Ok(v)  => println!("Success {v}"),		
//		Result::Err(e) => println!("Fail {e}"),

		Ok(v)  => println!("Success {v}"),		
		Err(e) => println!("Fail {e}"),
	}

//	if let Result::Ok(v) = ret {
	if let Ok(v) = ret {

	}	

	println!("{ret}"); // error
	println!("{ret:?}"); // ok
}
