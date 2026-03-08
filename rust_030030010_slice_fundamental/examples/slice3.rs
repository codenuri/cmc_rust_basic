fn main() {
	
	let start = 1;
	let end   = 4;

	let a : [i32;5] = [0, 1, 2, 3, 4];

	let r1 : &i32     = &a[0];
	let r2 : &[i32;5] = &a;
	let r3 : &[i32]   = &a[start..end];
	
	println!("{:p}", r1);	// 주소
	println!("{:p}", r2);	// 주소
	println!("{:p}", r3);	// 주소 + 갯수

	println!("{}", std::mem::size_of_val(&r1)); // 8
	println!("{}", std::mem::size_of_val(&r2)); // 8
	println!("{}", std::mem::size_of_val(&r3)); // 16
}
