#[derive(Debug)]
enum Color {
	Red, 
	Green, 
	Blue,
}

fn main() {

	let c : Color = Color::Red;
	let c         = Color::Red;

//	println!("{c}");	// error
	println!("{c:?}");	// ok. #[derive(Debug)] 필요

	match c {
		Color::Red   => println!("Red"),
		Color::Green => println!("Green"),
		Color::Blue  => println!("Blue"),
	}

	if let Color::Red = c {
		println!("Red");
	}
}


match c {
	Color::Red   => println!("Red"),
	Color::Green => println!("Green"),
	Color::Blue  => println!("Blue"),
}

if let Color::Red = c {
	println!("Red");
}