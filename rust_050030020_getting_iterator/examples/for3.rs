fn main() {

	let v = vec![1,2,3,4,5];
	let a = [1,2,3,4,5];

	for e in v {}
	for e in a {}

//	println!("{v:?}"); // error
	println!("{a:?}"); // ok
}
