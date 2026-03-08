#![allow(unused)]

fn add(a : i32, b : i32) -> i32 {
	a + b
}

fn main() {

	let f1 = add;
	let f2 = add as fn(i32, i32)->i32;

	println!("{}", std::any::type_name_of_val(&f1));
	println!("{}", std::any::type_name_of_val(&f2));

	let f3 : fn(i32, i32)->i32 = add;
}
