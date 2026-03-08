fn main() {

	let a = [0, 1, 2, 3, 4];

	let r : &[i32;5] = &a; 	
	let s : &[i32]   = &a;

	println!("{:p}", r);
	println!("{:p}", s);

	let s : &[i32]   = a; 	// error

	let r = &a;				// r : &[i32;5]
	let s = a.as_slice();	// s : &[i32]
}
