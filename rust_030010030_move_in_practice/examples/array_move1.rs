fn main() {

	let a1 = [1,2,3];
	let a2 = [String::from("A"), String::from("B")];

	let _a = a1;	// copy
	let _a = a2;	// move

	println!("{a1:?}");	// ok 
//	println!("{a2:?}");	// error
}
