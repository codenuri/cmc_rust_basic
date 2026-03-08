fn main() {

	let t1 = (1, 2.2, 'A');
	let t2 = (1,);
	let t3 = (1);
	let t4 = ();

	println!("{}", std::any::type_name_of_val(&t2));
	println!("{}", std::any::type_name_of_val(&t3));
	println!("{}", std::any::type_name_of_val(&t4));

	// 요소 접근 
	println!("{}, {}, {}", t1.0, t1.1, t1.2);

	let mut t4 = (1, 2, 3);
	t4.0 = 10; 		// ok
	t4 = (2, 3, 4);	// ok
//	t4 = (2, 3);	// error
//  t4.push(5);     // error

	// destructuring
	let t = (1, 2.2, 'A');
		
	let (a, b, c) = t;
	let (d, _, e) = t;
}
