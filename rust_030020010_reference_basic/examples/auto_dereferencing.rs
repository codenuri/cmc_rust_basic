struct Point {
	x : i32,
	y : i32,
}
fn main() {
	
	let n = 3i32;
	let r = &n;

	let a = r;	// a : &i32
	let b = *r;	// b : i32

	let r = &n;
	println!("{}", (*r).pow(2));//	9
	println!("{}", r.pow(2));  	// 	9  auto dereferencing

	let r = &Point{x:1, y:2};
	println!("{}", (*r).x);	// 1
	println!("{}", r.x);	// 1  auto dereferencing

	let r = &n;
	println!("{}", r); 	 // 대상체(n) 의 값
	println!("{:p}", r); // 대상체(n) 의 주소, r이 보관하는 값

}