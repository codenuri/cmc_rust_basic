fn main() {

	let mut v1 = vec![1,2,3,4,5];
	let mut v2 = vec![1,2,3,4,5];
	let mut v3 = vec![1,2,3,4,5];

	for e in v1 {
		println!("{}", std::any::type_name_of_val(&e));
	}
	for e in &v2 {
		println!("{}", std::any::type_name_of_val(&e));
	}	
	for e in &mut v3 {
		println!("{}", std::any::type_name_of_val(&e));
	}
	
	for e in v2.iter()     {}
	for e in v3.iter_mut() {}
}
