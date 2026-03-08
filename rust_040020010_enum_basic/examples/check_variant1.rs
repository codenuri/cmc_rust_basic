#[derive(Copy, Clone, Debug)]
enum Color { Red, Green, Blue, }

fn main() {
	let mut c : Color = Color::Red;

	match c {
		Color::Red   => println!("Color::Red"),
		Color::Green => println!("Color::Green"),
		Color::Blue  => println!("Color::Blue"),
	} 

	if let Color::Red = c {
		println!("Red");
	}

	if matches!(c, Color::Red) {
		println!("Red");
	}	
}
