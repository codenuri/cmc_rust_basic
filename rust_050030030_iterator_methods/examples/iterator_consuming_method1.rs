fn main() {

	let v = vec![1, 2, 3, 4, 5];
	
	let it = v.iter();

	let s : i32 = it.sum();	// 15	

	let s : i32 = it.sum();	// error
							// value used here after move

	let s1 : i32 = v.iter().sum();		
	let s2 : i32 = v.iter().product();	

	println!("{s1}"); // 15
	println!("{s2}"); // 120
}
