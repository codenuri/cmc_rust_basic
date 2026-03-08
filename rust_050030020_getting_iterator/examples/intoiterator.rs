fn main() {

	let mut v1 = vec![1,2,3,4,5];
	let mut v2 = vec![1,2,3,4,5];
	let mut v3 = vec![1,2,3,4,5];

	let mut it1 = v1.into_iter();
	let mut it2 = (&v2).into_iter();    // (&v2).iter()
										// v2.iter()
	let mut it3 = (&mut v3).into_iter();// v3.iter_mut()

//	println!("{v1:?}"); // error
	println!("{v2:?}"); // ok
	println!("{v3:?}"); // ok
}
