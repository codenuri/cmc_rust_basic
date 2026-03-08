fn main() {
	let mut s = String::new();

	let sin : std::io::Stdin = std::io::stdin();

	let ret = sin.read_line(&mut s);
	
	// ret 는 Result<usize, std::io::Error> 타입
	match ret {
		Ok(len) => {}, // len 은 읽은 문자열 길이
		Err(e)  => {}
	}
	let s = s.trim(); // 앞뒤 공백 제거
	println!("{s}");
	//------------------------------
	let mut s = String::new();
	let ret = std::io::stdin().read_line(&mut s).unwrap();

	let s = s.trim();	
	println!("{s}");	
}