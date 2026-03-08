fn do_something() {}

fn main()
{
	let b = true;
	match b {
		true  => println!("true"),
		false => println!("false"),
	}

	let n = 1i32;
	match n {
		1 => println!("one"),		
		2 => println!("two"),
//		_ => println!("other"),	// 이 부분 생략시 컴파일 에러
	}

	match n {
		_ => println!("other"),
		1 => println!("one"),	// warning				
	}

	match n {
		1 => println!("one"),		
		2 => { println!("two"); do_something(); }		
		3 => { println!("three");} 	
		_ => { }
	}	
}
