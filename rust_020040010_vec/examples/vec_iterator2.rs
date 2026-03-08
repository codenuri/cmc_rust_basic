fn main() {

	let v = vec![1, 2, 3, 4, 5];

	println!("{}", v.iter().sum::<i32>() );
	println!("{}", v.iter().product::<i32>() );

	for i in v.iter().rev().step_by(2) {

		print!("{}, ", i); // 5, 3, 1
	}
	println!();

	let v = v.iter().rev().step_by(2).collect::<Vec<_>>();

	println!("{v:?}"); // [5, 3, 1]
}

