fn main() {

	let a = [String::from("A"), String::from("B")];

	let s = a[0];			// error
	let s = a[0].clone();	// ok
	let s = &a[0];			// ok

}