fn main() {

    let ret = std::fs::read_to_string("hello.txt"); 
							// ret : Result<String, io::Error>

	match ret {
        Ok(s)  => println!("{s}"),
        Err(e) => println!("read failed: {e}"),
    }


	if let Ok(s) = ret {
		println!("using if let : {s}");
	}

//	let s = ret.unwrap();

//	let s = std::fs::read_to_string("hello.txt").unwrap();
}
