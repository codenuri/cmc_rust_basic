fn main() {

	let arr : [i32;5] = [1, 2, 3, 4, 5];
	let arr 		  = [1, 2, 3, 4, 5];
	
	// 요소의 접근 []
	let x = arr[0];

	println!("{arr[0]}"); 	// error
	println!("{}", arr[0]);	// ok	

	// 모든 요소 출력
	println!("{arr}"); 		// error
	println!("{arr:?}");	// ok
	println!("{arr:#?}");	// ok
}