
fn f1(x: &i32) -> &i32 { 
	x
}
// f1 과 f2 는 동일한 의미
fn f2<'a>(x: &'a i32) -> &'a i32 { 
	x
}

//         'a      'a
fn f3(x : &i32) -> &i32 {
	static N : i32 = 0;
	&N
}

fn f4(x : &i32) -> &'static i32 {
	static N : i32 = 0;
	&N
}
fn main(){

	let r1;
	let r2;
	{
		let n = 0;
		r1 = f3(&n);
		r2 = f4(&n);
	}
	println!("{r1}"); // error
	println!("{r2}"); // ok
}



