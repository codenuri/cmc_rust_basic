fn main() {
	
	let num = 15;

	// #1. match
	match num {
		10..=20 => println!("10 ~ 20"),
		_       => {}
	}

	// #2. for
	for i in 1..10 {
		print!("{i}, ")
	}
	println!();	

	for i in 'a'..'z' {
		print!("{i}, ")
	}
	println!();	

	// #3. slice
	let mut arr = [9,8,7,6,5,4,3,2,1];
	arr.sort();
	arr[1..4].sort();
	
	// #4. method argument
	let mut s = String::from("ABCDEFG");
	s.drain(1..4);
	println!("{s}");

	foo(1..4);

	// #5. using Range method
	println!("{}", (0..10).contains(&3));	
	println!("{}", (0..=10).sum::<i32>());
}

fn foo( r : std::ops::Range<i32> ) {

	println!("{}", r.start);
	println!("{}", r.end);
}
