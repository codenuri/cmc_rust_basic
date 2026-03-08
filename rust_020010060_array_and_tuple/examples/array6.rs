fn main() {
	
	let mut arr = [1,2,3,4,5];

	for e in arr  {
		print!("{e}, ");
	}
	println!();

	for e in arr.iter().step_by(2) {
		print!("{e}, ");
	}
	println!();

	for e in &arr[1..4]  {
		print!("{e}, ");
	}
	println!();
	
	for e in &mut arr {
		*e = 0;
	}
	println!("{arr:?}");
}