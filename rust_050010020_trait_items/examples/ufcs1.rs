trait MyTrait {
	fn m1(&self);
}

struct MyType;

impl MyType {
	fn m1(&self) { println!("MyType m1");}
//	fn m1(&self) { } // error
}

impl MyTrait for MyType {
	fn m1(&self) { println!("MyTrait m1");}
}

fn main() {
	let m = MyType{};

	m.m1();
	MyTrait::m1(&m);
}
