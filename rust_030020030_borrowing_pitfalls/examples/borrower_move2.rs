fn my_swap<T>(x: &mut T, y: &mut T) {
    unsafe {
        let tmp: T = std::ptr::read(x);
        std::ptr::write(x, std::ptr::read(y));
        std::ptr::write(y, tmp);
    }
}

fn main() {
	
	let mut x = String::from("AAA");
	let mut y = String::from("BBB");

	my_swap(&mut x, &mut y);

	println!("{x}, {y}");
}

