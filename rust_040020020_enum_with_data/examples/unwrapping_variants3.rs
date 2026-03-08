#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Command {
	Quit,			 
	Run(i32),
	Move(i32, i32),
}

fn main() {

	let c = Command::Run(3);

	if Command::Run(3) == c { 	// True
//	if Command::Run(6) == c { 	// False
//	if Command::Run(x) == c {   // error
//	if Command::Run    == c {	// error
//	if Command::Run(..)== c { 	// error
//	if Command::Run(_) == c { 	// error
		println!("True");
	}
	else {
		println!("False");
	}
}

if Command::Run(3) == c {} 	// True
if Command::Run(6) == c {} 	// False
if Command::Run(x) == c {}  // error
if Command::Run    == c {}	// error
if Command::Run(..)== c {} 	// error
if Command::Run(_) == c {} 	// error
if let Command::Run(_) == c{}// ok	