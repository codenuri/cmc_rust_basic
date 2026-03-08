fn main() { 

	let mut v = vec![1,2,3,4,5];

	let r = &mut v; 	// <= Start mutable borrowing

	println!("{:?}", v.len()); // error
	println!("{:?}", v.pop()); // error
	println!("{:?}", r.pop()); // Vec::<i32>::pop(&mut r)
								// reborrow .. ok
								
	println!("{r:?}"); 	// <= End mutable borrowing
}
