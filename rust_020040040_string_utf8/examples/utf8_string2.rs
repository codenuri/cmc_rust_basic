fn main() {

	let mut s = String::from("A가BC");

	// #1
	println!("{:?}", s.find('B')); // Some(4)

	// #2. 
	s.insert(1, 'X'); // ok
//	s.insert(3, 'X'); // panic!()

	s = String::from("A가BC");
//	let c  = s[4];	   // error    s[index] 는 안됨
	let s1 = &s[0..4]; // ok "A가" s[range] 는 가능 
//	let s2 = &s[0..3]; // panic!()     "&str" 강의 참고

	// #3. 
	println!("{}", s.is_char_boundary(1));
	println!("{}", s.is_char_boundary(3));
}
