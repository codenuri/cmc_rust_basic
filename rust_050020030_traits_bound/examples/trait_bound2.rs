fn foo<T: std::fmt::Display >(v: &T) {
	println!("{}", v);	
}

struct Data<T:Copy> {
	value : T
}

fn main() {

	let d1 = Data::<i32>   { value: 10 };  // ok
	let d2 = Data::<String>{ value: "A" }; // error
}
