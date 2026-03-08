#[derive(Debug, Copy, Clone)]
enum Command {
	Move1(i32,   i32),
	Move2{x:i32, y:i32},
}

fn main() {

	let mut c;
	c = Command::Move1(3, 7);
	c = Command::Move2{x:3, y:7};

	match c {	
		Command::Move1(a, b)     => println!("Move1 {a}, {b}"),
//		Command::Move2{x:a, y:b} => println!("Move2 {a}, {b}"),
		Command::Move2{y:a, x:b} => println!("Move2 {a}, {b}"),
	}
}
