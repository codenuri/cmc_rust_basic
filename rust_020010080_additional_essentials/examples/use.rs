fn main() {

	let t1: std::time::Instant = std::time::Instant::now();
	let t2                     = std::time::Instant::now();

	// use
	use std::time::Instant;	
	let t3 = Instant::now();

	use std::time::{Instant, Duration};
	use std::time::*;

	// prelude
	let s1 = std::string::String::from("hello");
	let s2 = String::from("hello");
}
