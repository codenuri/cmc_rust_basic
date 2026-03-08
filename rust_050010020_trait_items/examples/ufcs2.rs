struct MyType;

trait MyTraitA {
	fn m1(&self);
}
trait MyTraitB {
	fn m1(&self);
}

impl MyTraitA for MyType {
	fn m1(&self) { println!("MyTraitA m1");}
}
impl MyTraitB for MyType {
	fn m1(&self) { println!("MyTraitB m1");}
}


fn main() {
	let m = MyType{};

//	m.m1();	// error. disambiguate the method
	MyTraitA::m1(&m);
	MyTraitB::m1(&m);
}
