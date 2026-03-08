fn main() {

	let mut v = vec![1];

	let ret1 = v.pop(); 
	let ret2 = v.pop();

	println!("{ret1:?}"); // Some(1)
	println!("{ret2:?}"); // None

	println!("{}", std::any::type_name_of_val(&ret1));
							// Option<i32>

	match v.pop() {
		Some(e) => println!("removed element : {e}"),
		None    => println!("empty vec"),
	}
}
