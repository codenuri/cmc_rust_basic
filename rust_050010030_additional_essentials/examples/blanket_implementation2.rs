trait SayHello {
	fn hello(&self);
}

// error. negative trait bound 없음
impl<T: !Copy > SayHello for T {
	fn hello(&self) { }
}

// error. Coherence Rule Violation
impl<T> SayHello for T {
	fn hello(&self) { }
}
impl<T:Copy> SayHello for T {
	fn hello(&self) { }
}

fn main() {

}