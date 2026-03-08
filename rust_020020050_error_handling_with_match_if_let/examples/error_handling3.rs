#[derive(Debug)]
enum Result<T, E> {
	Ok(T),
	Err(E),
}

fn foo() -> Result<i32, i32> {

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
