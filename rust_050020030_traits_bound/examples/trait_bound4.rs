#![allow(unused)]

trait Print {
	fn print(&self);
}
struct TypeInfo<T>(T);

fn f1<T:Print>(v: T) {	

	let ti : TypeInfo<T> = TypeInfo(v);
//	ti.print(); // error
}

fn f2<T>(v: T) 
	where TypeInfo<T>: Print
{	
	let ti : TypeInfo<T> = TypeInfo(v);
	ti.print(); // ok
}

fn main() {
	f2( 10 );
}

impl Print for TypeInfo<i32> {
	fn print(&self) {
		println!("print i32 type information");
	}
}