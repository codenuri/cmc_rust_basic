fn main() {
	
	let mut a = [1, 3, 5, 7, 9, 2, 4, 6, 8, 10];

	let s = &a[2..7]; // ok.    &a[2..7] => &[i32] 타입
	let n = a[2..7];  // error	a[2..7]  =>  [i32] 타입
//	let n = *s;		  // error  *s       =>  [i32] 타입

	println!("{}", (*s).len()); 	// ok
	println!("{}", a[2..7].len());	// ok
	println!("{}", s.len());		// ok

	a[2..7].sort();
	a[2..7].reverse();

//	for e in a[2..7] {	// error
	for e in &a[2..7] {	// ok
	}
}


