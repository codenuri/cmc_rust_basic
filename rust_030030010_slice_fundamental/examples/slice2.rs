fn main() {
	
	let a : [i32;5] = [0, 1, 2, 3, 4];

	let r1 = &a[0];
	let r2 = &a;
	let r3 = &a[1..4];

	let r1 : &i32     = &a[0];
	let r2 : &[i32;5] = &a;
	let r3 : &[i32]   = &a[1..4];
	
	println!("{r1:?}");
	println!("{r2:?}");
	println!("{r3:?}");
}
