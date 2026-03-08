enum Color   { Red,  Green,    Blue, }
enum Command { Quit, Run(i32), Move(i32, i32), }

fn main() {
	
	let mut c1 = Color::Red;
	c1 = Color::Green;
	c1 = Color::Blue;

	let mut c2 = Command::Quit;
	c2 = Command::Run(30);
	c2 = Command::Move(10, 20);

	println!("{}", std::mem::size_of_val(&c1)); // 1
	println!("{}", std::mem::size_of_val(&c2)); // 12
}
