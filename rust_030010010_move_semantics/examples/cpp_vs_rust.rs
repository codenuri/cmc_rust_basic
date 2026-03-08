// C++
int main() {
	string s1 = "to be or not to be";
	string s2 = "to be or not to be";

	string s3 = s1;			// deep copy
	string s4 = move(s2);	// move

	string v = s2; // ok. move 된 후 s2 사용가능
}

// Rust 
fn main() {
	let s1 = String::from("to be or not to be");
	let s2 = String::from("to be or not to be");

	let s3 = s1; 			// move
	let s4 = s2.clone();	// deep copy

	let v = s1; // error. move 된 후 사용못함
}