#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Color { Red, Green, Blue, }

fn main() {
	let c = Color::Red;

	if c == Color::Red {
		println!("Red");
	}
}
