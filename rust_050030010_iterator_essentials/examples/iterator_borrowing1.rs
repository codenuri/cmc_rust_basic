fn main() {

	let mut v = vec![1, 2, 3, 4, 5];
	
	let mut it = v.iter();		// <= Start Immutable Borrowing

	println!("{:?}", v.len());	// ok		Vec::len(&v)
//	println!("{:?}", v.pop());	// error	Vec::pop(&mut v)

	println!("{:?}", it.next());// <= End Immutable Borrowing


	let mut it = v.iter_mut();	// <= Start Mutable Borrowing

//	println!("{:?}", v.len());	// error	Vec::len(&v)
//	println!("{:?}", v.pop());	// error	Vec::pop(&mut v)

	println!("{:?}", it.next());// <= End Mutable Borrowing	
}
