fn main() {
	
	let ret = factorial(5);

	println!("{ret}");

	
	fn factorial(x : i32) -> i32 {
		if x == 1 {x} else { x*factorial(x-1) }
	}
}
