fn main() {

	let t1 = (5, 3.4);					// Copy Type
	let t2 = (5, String::from("ABC"));	// Non-Copy Type

	let _t = t1;	// copy
	let _t = t2;	// move

	println!("{t1:?}");	// ok
	println!("{t2:?}");	// error
}
