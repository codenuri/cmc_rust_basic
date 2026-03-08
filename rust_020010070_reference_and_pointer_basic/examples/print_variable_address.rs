fn main() {

	let n = 10i32;

	println!("{}", n);		// 10
	println!("{}", &n);		// 10
	println!("{:p}", &n);	// n 의 주소


	let r = &n;

	println!("{}", *r);
	println!("{}",  r);
	println!("{:p}", r);
	println!("{:p}", &r);
	println!("{}",   &r); 
}

