// 1. C-like enum
enum Color { 
	Red, 
	Green, 
	Blue, 
}
impl Color {

	// 1. methods
	// 2. associated functions
	// 3. associated constants
}
// 2. enum with data
enum Message {
    Quit,                	// Data-less   variant
    Write(String),       	// tuple-like  variant
	Move{x:i32, y:i32},     // struct-like variant    
}

fn main() {
}
