fn main() {

	let arr = [1,2,3,4,5];

	let it = arr.iter();

	for i in it {
		print!("{i}, ");
	}
	println!();

	
	for i in arr {
		print!("{i}, ");
	}
	println!();
}