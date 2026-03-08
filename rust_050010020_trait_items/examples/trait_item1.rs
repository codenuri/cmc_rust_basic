trait MyTrait {

	fn required_method(&self);

	fn provided_method1(&self) { 
		println!("MyTrait provided_method1 default impl");
	}
	fn provided_method2(&self) { 
		println!("MyTrait provided_method2 default impl");
	}

	fn required_associated_function();

	fn provided_associated_function1() { 
		println!("MyTrait provided_asfn1 default impl");
	}
	fn provided_associated_function2() { 
		println!("MyTrait provided_asfn2 default impl");
	}
}


struct MyType;

impl MyTrait for MyType {

	fn required_method(&self) { 
		println!("MyType  required_method");
	}
	fn provided_method2(&self) { 
		println!("MyType  provided_method2");
	}

	fn required_associated_function() {
		println!("MyType  required_associated_function");
	}

	fn provided_associated_function2() { 
		println!("MyType  provided_associated_function");
	}	
}

fn main() {

	let mt = MyType{};

	mt.required_method();
	mt.provided_method1();	// MyTrait 기본 구현 사용
	mt.provided_method2();

	MyType::required_associated_function();
	MyType::provided_associated_function1(); // MyTrait 기본 구현 사용
	MyType::provided_associated_function2();
}
