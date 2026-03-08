fn main() {

	let arr = [1,2,3,4,5];

	// #1. 모든 요소 열거
	for i in arr {
		print!("{i}, ");	// 1, 2, 3, 4, 5,
	}
	println!();

	for i in arr.iter() {
		print!("{i}, ");	// 1, 2, 3, 4, 5,
	}
	println!();

	// #2. adapter 적용
	for i in arr.iter().rev() {
		print!("{i}, ");	// 5, 4, 3, 2, 1,
	}
	println!();	

	for i in arr.iter().rev().step_by(2) {
		print!("{i}, ");	// 5, 3, 1,
	}
	println!();		

	for t in arr.iter().enumerate() {
		print!("{t:?}, ");	// (0, 1), (1, 2), (2, 3), 
							// (3, 4), (4, 5),
	}
	println!();	

	println!("{}", arr.iter().sum::<i32>());

//	for i in arr.rev() {		// error
	for i in arr.iter().rev() {	// ok
		print!("{i}, ");	
	}
	println!();		
}