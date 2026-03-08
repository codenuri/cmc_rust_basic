fn main() {

	let r1;
	let r2;

	let long = 1;
	{
		let short = 1;

		r1 = &long;
		r2 = &short;
	}
	println!("{r1}"); // ok
//	println!("{r2}"); // error
					  // borrowed value does not live long enough
}

