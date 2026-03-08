fn main() {

	let arr = [10, 20, 30, 40, 50];

//	let ret1 = arr[7];

	let ret2 = arr.get(7);
	let ret3 = arr.get(3);

	println!("{ret2:?}");	// None
	println!("{ret3:?}");	// Some(40)


	match ret2 {
		Some(v) => println!("7번째 요소는 {v}"),
		None    => println!("7번째 요소 없음"),
	};
}