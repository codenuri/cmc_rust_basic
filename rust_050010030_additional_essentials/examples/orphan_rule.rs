trait Hello {
	fn hello(&self);
}

struct MyType;

// user trait + user type => ok
impl Hello for MyType {
	fn hello(&self) {}
}

// user trait + standard type => ok
impl Hello for String {
	fn hello(&self) {}
}

// standard trait + user type => ok 
impl Default for MyType {
	fn default() -> Self { MyType{} }
}

// standard trait + standard type => error
impl Default for Vec<i32> {
	fn default() -> Self { vec![1, 2, 3] }
}

fn main() {
}

