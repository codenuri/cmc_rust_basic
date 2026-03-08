fn swap_i32(a : &mut i32, b : &mut i32) {
	let t = *a;
	*a = *b;
	*b = t;
}

fn swap_string(a : &mut String, b : &mut String) {
	let t = *a;	// error
	*a = *b;
	*b = t;
}

fn main() {	
	let mut s1 = String::from("AAA");
	let mut s2 = String::from("BBB");

	std::mem::swap(&mut s1, &mut s2);

	println!("{s1}, {s2}");
}
