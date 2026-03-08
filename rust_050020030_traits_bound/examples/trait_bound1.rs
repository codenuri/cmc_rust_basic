fn f1<T>(v: &T) {

	let n = v; // ok
//	println!("{}", v);	// error
}

fn f2<T: std::fmt::Display >(v: &T) {
	println!("{}", v);	// ok
}

fn main() {

	let n = 10;
	let a = [1,2,3,4,5];

	f1(&n);	// ok  n : i32
	f1(&a);	// ok  a : [i32;5]

	f2(&n);	// ok
//	f2(&a);	// error
}
