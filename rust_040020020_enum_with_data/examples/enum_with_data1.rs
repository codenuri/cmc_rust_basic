#[derive(Debug, Copy, Clone)]
enum Command {
	Quit,			 
	Run(i32),
	Move(i32, i32),
}

fn main() {
	
	let c1 = Command::Quit;
	let c2 = Command::Run(30);
	let c3 = Command::Move(10, 20);

	println!("{c1:?}");
	println!("{c2:?}");
	println!("{c3:?}");
}
