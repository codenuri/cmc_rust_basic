struct MyType;

trait GenericTrait<T> {
	fn m1(&self) -> T;
}

impl GenericTrait<i32> for MyType {
	fn m1(&self) -> i32 { 0 }
} 

impl GenericTrait<f64> for MyType {
	fn m1(&self) -> f64 { 0.0 }
} 

fn main() {
	
	let m = MyType{};
	
//	m.m1(); // error	
	GenericTrait::<i32>::m1(&m);
	GenericTrait::<f64>::m1(&m);
}
