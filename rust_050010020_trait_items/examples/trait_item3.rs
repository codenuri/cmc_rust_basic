struct MyType;

trait GenericTrait<T> {
	fn method(&self) -> T;
}

impl GenericTrait<i32> for MyType {
	fn method(&self) -> i32 { 0 }
} 

impl GenericTrait<f64> for MyType {
	fn method(&self) -> f64 { 0.0 }
} 

fn main() {
	
	let m = MyType{};
	
//	m.method(); // error
	
	GenericTrait::<i32>::method(&m);
	GenericTrait::<f64>::method(&m);
}
