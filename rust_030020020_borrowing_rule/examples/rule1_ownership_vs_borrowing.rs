fn main() {

	let mut n = 10;
	let mut s = String::from("ABC");
//  let mut x = s; // move 를 통한 "ABC"의 소유자 변경

	let r1 = &mut n;
	let r2 = &mut s;

	let _n = *r1; // ok
	let _s = *r2; // error 대여자는 move 할수 없다
	let _s = r2.clone(); // ok

	*r1 = 20;					  // ok
	r2.replace_range(1..3, "ab"); // ok

	println!("{n}");
	println!("{s}");
}