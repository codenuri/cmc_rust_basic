struct MyType1 {
	x : i32,				// Send Trait 조건 만족
}

struct MyType2 {
	x : std::rc::Rc<i32>,	// Send  Trait 조건 만족 안함
}

fn foo< T:Send >(_ : T) {
}

fn main() {

	let mt1 = MyType1{x : 10};
	let mt2 = MyType2{x : std::rc::Rc::new(10)};

	foo(mt1);	// ok
	foo(mt2);	// error
}
