fn main() {

	let a = [1, 2, 3, 4];
	let v = vec![1, 2, 3, 4];

	println!("{}", std::mem::size_of_val(&a)); 
					// 16 : 4 * 4(i32)

	println!("{}", std::mem::size_of_val(&v)); 
				    // 24 : pointer, usize, usize 

	println!("{:p}", v.as_ptr());	// 버퍼 주소
	println!("{}",   v.len());		// 4
	println!("{}",   v.capacity());	// 4
}
