fn swap( a :&mut i32, b :&mut i32 ) {

	let tmp = *a;
	*a = *b;
	*b = tmp;
}

fn main() {
	
	let mut x = 10;
	let mut y = 20;

	swap(&mut x, &mut y);

	println!("{x}, {y}"); // 20, 10
}
