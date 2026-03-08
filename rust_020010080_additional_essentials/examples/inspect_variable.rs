fn main() {

	let v : i32 = 10;

	println!("{}", v);
	println!("{:p}", &v);

	println!("{}", std::mem::size_of_val(&v));
	println!("{}", std::mem::size_of::<i32>());

	println!("{}", std::any::type_name_of_val(&v));
	println!("{}", std::any::type_name::<i32>());

}