fn main() {

	let mut a = [1, 2, 3, 4, 5];
	let mut v = vec![1, 2, 3, 4, 5];

	a[0] = 10;
	v[0] = 10;

//	a.push(6); 	// error
	v.push(6);	// ok

//	a.resize(10, 0); // error
	v.resize(10, 0); // ok	
	
	println!("{v:?}");	// [10, 2, 3, 4, 5, 6, 0, 0, 0]
	println!("{a:?}");  // [10, 2, 3, 4, 5]
}




