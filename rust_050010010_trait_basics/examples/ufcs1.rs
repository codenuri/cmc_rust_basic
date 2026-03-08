trait MyTrait {

	fn method(&self, arg : i32);
}

struct MyType;

impl MyTrait for MyType {

	fn method(&self, arg : i32){
		println!("method({arg})");
	}
}

fn main() {

	let m = MyType{};

	m.method(3);

	MyType::method(&m, 3);
	MyTrait::method(&m, 3);
	<MyType as MyTrait>::method(&m, 3);
}