struct MyType1;
struct MyType2;
struct MyType3;

trait MyTrait1 {

    fn m1(&self) -> i32;
	fn m2(&self) -> Self;
}

impl MyTrait1 for MyType1 {
	
    fn m1(&self) -> i32     { 10 }
//	fn m2(&self) -> Self    { MyType1{} }
	fn m2(&self) -> MyType1 { MyType1{} }
}






// generic 
trait MyTrait2<T> {
	fn m1(&self) -> T;
}

impl MyTrait2<i32> for MyType2 {
	
    fn m1(&self) -> i32 { 10 }
}

// associated type
trait MyTrait3 {
	
	type Output;

	fn m1(&self) -> Self::Output;
}

impl MyTrait3 for MyType3 {

	type Output = i32;
    fn m1(&self) -> Self::Output { 10 }
}









fn main() {
	
}



