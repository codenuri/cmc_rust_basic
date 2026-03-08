// chaining
fn main() {

	let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

	for e in v.iter().rev() {
		print!("{e}, "); // 10,9,8,7,6,5,4,3,2,1
	}
	println!();

	for e in v.iter().rev().take(4) {
		print!("{e}, ");	// 10, 9, 8, 7
	}
	println!();

	for e in v.iter().rev().take(5).filter(|e| *e % 2==0) {
		print!("{e}, ");	// 10, 8, 6
	}	
	println!();

	let s : i32 = v.iter().rev().take(5).filter(|e| *e % 2==0).sum();

	println!("{}", s ); // 24
}