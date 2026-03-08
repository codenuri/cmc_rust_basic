#[derive(Debug, Copy, Clone)]
enum Command {
	Quit,			 
	Run(i32),
	Move(i32, i32),
}

fn main() {

	let c = Command::Move(3, 7);

	// using match
	match c {	
		Command::Quit       => println!("Quit"),
		Command::Run(s)     => println!("Run {s}"),
		Command::Move(x, y) => println!("Move {x}, {y}"),
	}

	match c {	
		Command::Quit       => println!("Quit"),
		Command::Run(_)     => println!("Run"),
		Command::Move(0, y) => println!("Move 0, {y}"),
		Command::Move(1, _) => println!("Move 1"),
		_ => {},
	}

	// using if let
	if let Command::Move(x, y) = c { }
	if let Command::Move(1, y) = c { }
	if let Command::Move(_, y) = c { }

	if let Command::Move(_, _) = c { }
	if let Command::Move(..)   = c { }
	
//	if let Command::Move = c  { }	// error
	if let Command::Quit = c  { }   // ok	


	// using let ~ else
	let Command::Move(x, y) = c else { panic!(); };

	match c {
		Command::Move(x, y) => { }, 
		_ => panic!(), 
	}
}
