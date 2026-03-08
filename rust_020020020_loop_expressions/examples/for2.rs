fn main()
{
	// #1
	for i in 0..10 {
		print!("{}, ", i);
	}
	println!();

	// #2
	for i in (0..10).rev() {
		print!("{}, ", i);
	}
	println!();

	// #3
	let arr = [1, 2, 3, 4, 5];

	for e in arr {
		print!("{}, ", e);
	}
	println!();

	// #4
	for e in arr.iter().rev() {
		print!("{}, ", e);
	}
	println!();

	// #5
	for e in &arr[1..4] {
		print!("{}, ", e);
	}
	println!();

	// #6
	for e in arr[1..4].iter().rev() {
		print!("{}, ", e);
	}
	println!();
}
