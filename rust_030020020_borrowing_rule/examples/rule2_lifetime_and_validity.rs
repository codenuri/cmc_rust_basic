fn main() {

	let r;
	{
		let n = 10;
		r = &n;
		println!("{}", *r);
	}
//	println!("{}", *r); // compile-time error
}

