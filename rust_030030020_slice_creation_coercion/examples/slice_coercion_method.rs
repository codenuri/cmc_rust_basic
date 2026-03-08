fn main() {

	let mut a = [1, 3, 5, 2, 4];

	a.sort();

	println!("{a:?}");

	let s1 : &[i32] = &a; // ok
	let s2 : &[i32] = a;  // error
}
