fn main() {

	let a1 : [i32;3]         = [1, 2, 3];
	let t1 : (u8, f64, char) = (1, 2.2, 'A');

	let _a2 = [1, 2, 3];
	let _t2 = (1, 2.2, 'A');
	let _t3 = (1u8, 2.2f64, 'A');


	println!("{}", std::mem::size_of_val(&a1)); // 12
	println!("{}", std::mem::size_of_val(&t1)); // 16

	println!("{a1:?}");
	println!("{t1:?}");
}
