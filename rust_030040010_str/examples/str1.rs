fn main() {

	let v  = vec![1i32, 2, 3, 4, 5];
	let s1 = String::from("ABCDE");
	let s2 = String::from("A가B");

	let sc1 = v.as_slice();
	let sc2 = unsafe { s1.as_bytes() };
	let sc3 = s2.as_str() ;

	println!("{}", std::any::type_name_of_val(&sc1));
	println!("{}", std::any::type_name_of_val(&sc2));
	println!("{}", std::any::type_name_of_val(&sc3));

	println!("{}", std::mem::size_of_val(&sc1));
	println!("{}", std::mem::size_of_val(&sc2));
	println!("{}", std::mem::size_of_val(&sc3));

	println!("{sc1:p}");
	println!("{sc2:p}");
	println!("{sc3:p}");
}