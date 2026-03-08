fn f1(s : &mut String) {
	// s 사용시 borrowing rule 에 적용 받게 된다.
}

fn f2(s : String ) -> String {
	// s 는 소유자. s 사용에 제한 없음. 사용후 소유권 이전
	s 
}

fn main() {

	let mut s1 = String::from("ABCD");
	let mut s2 = String::from("ABCD");

	f1(&mut s1);
	s2 = f2(s2);

	// s1, s2 모두 계속 사용가능
	println!("{s1}, {s2}");
}
