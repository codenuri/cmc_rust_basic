fn main() {

	let f1 : f64 = 0.0 / 0.0;	// NaN (Not a Number)
	let f2 : f64 = 2.1;
	
	println!("{:?}", f1.partial_cmp(&f2));

	println!("{}", f1 < f2);	// false
	println!("{}", f1 > f2);	// false
	println!("{}", f1 <= f2);	// false
	println!("{}", f1 >= f2);	// false

	let n1 : i32 = 10;
	let n2 : i32 = 20;

	println!("{:?}", n1.cmp(&n2));			// ok
	println!("{:?}", n1.partial_cmp(&n2));	// ok
	
//	println!("{:?}", f1.cmp(&f2));			// error
	println!("{:?}", f1.partial_cmp(&f2));	// ok
}
