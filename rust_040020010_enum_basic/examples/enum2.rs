#[derive(Copy, Clone, Debug)]
enum Color { 
	Red, 
	Green, 
	Blue, 
}

fn main() {
	
	let c1 : Color = Color::Red;
	let c2         = Color::Green;
	let mut c3     = Color::Green;

	use Color::*;
	c3 = Red;

	println!("{c3:?}");
}

