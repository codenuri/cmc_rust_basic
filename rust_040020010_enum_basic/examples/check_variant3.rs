#[derive(Copy, Clone, Debug)]
enum Color { Red, Green, Blue, }

impl Color {
	fn is_red(&self) -> bool {
		match self {
			Color::Red => true,
			_ 		   => false,			
		}
	}
}

fn main() {
	let c1 = Color::Red;
	let c2 = Color::Blue;

	println!("{}", c1.is_red() );
	println!("{}", c2.is_red() );
}
