trait MyTrait {
	fn method(&self, arg1 : i32, arg2 : f64);
}

struct MyType;

impl MyTrait for MyType {
	fn method(&self, arg1 : i32, arg2 : f64) {}
}

fn main() {
	let m = MyType{};

	m.method(1, 3.4);	

	<MyType as MyTrait>::method(&m, 1, 3.4); // UFCS, FQS

	MyTrait::method(&m, 1, 3.4); // UFCS Simplied Form
	MyType::method(&m, 1, 3.4);  // UFCS Simplied Form
}
