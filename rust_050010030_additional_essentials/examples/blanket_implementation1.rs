trait SayHello {
	fn hello(&self);
}

//impl<T> SayHello for T {
impl< T:Copy > SayHello for T {
	fn hello(&self) {
		println!("hello");
	}
}

struct Point;

fn main() {

	3.hello();		// ok
	3.4.hello();	// ok
	Point{}.hello();// error
}