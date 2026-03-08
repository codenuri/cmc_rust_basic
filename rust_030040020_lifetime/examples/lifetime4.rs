#![allow(unused)]

// 인자가 한개인 경우
fn f1(arg : &i32) -> &'static i32 {
	static N : i32 = 10;	
	&N
}

fn f2<'a>(arg : &'a i32) -> &'a i32 {
	arg
}

fn main(){

	let n = 0;

	let r1 = f1(&n);
	let r2 = f2(&n);

}
