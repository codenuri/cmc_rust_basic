fn main() {

	let arr = [1i32, 2, 3];

	println!("{}", std::mem::size_of_val(&arr)); // 12

	let mut x = arr;	
	x = [0, 0, 0];
//	x = [0, 0, 0, 0];	 // error
//	x = [1.1, 2.2, 3.3]; // error
}

