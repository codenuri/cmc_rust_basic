#[derive(Debug)]
enum Result {
	Ok,
	Err,
}

fn foo() -> Result {

	Result::Ok
//	Result::Err
}

fn main() {

	let ret = foo();

	match ret {		
		Result::Ok  => println!("Success"),		
		Result::Err => println!("Fail"),
	}

	if let Result::Ok = ret {

	}
}