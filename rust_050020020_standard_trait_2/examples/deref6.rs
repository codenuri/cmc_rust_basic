fn main() {

	let v = vec![1,2,3,4,5];

	println!("{:?}", v); // ok
	println!("{}",   v); // error
}


// error : orphan rule 위반
impl<T> std::fmt::Display for Vec<T> {
	// ......
}