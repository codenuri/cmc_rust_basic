fn main() { 

	let mut v = vec![1,2,3,4,5];

	let r = &v;			// <== Start immutable Borrowing

	println!("{:?}", v.len());	// ok
	println!("{:?}", v.pop());	// error

	println!("{r:?}");	// <== End immutable Borrowing
}
