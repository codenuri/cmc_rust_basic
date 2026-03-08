#[derive(Debug)]
enum Result {
	Ok(i32),
	Err(i32),
}

fn foo() -> Result {

	Result::Ok(300)
//	Result::Err(100)
}

fn main() {

	let ret = foo();

	match ret {		
		Result::Ok(v)  => println!("Success {v}"),		
		Result::Err(e) => println!("Fail {e}"),
	}

	if let Result::Ok(v) = ret {

	}
}