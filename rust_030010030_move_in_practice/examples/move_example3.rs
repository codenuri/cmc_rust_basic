fn main() {
	let age = 30;

	let name = String::from("alice");
	
	if age >= 18 {
		println!("...");
	}
	else {
		let s = name;	// move 발생
//		return;			// A
	}

	println!("{name}");	// error
						// 단 A 가 있다면 에러 아님
}
