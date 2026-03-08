fn main()
{
	// 배열의 초기화
	let arr = [0, 0, 0, 0, 0];
	let arr = [0;5];
	let arr = [0i32;5];

	let arr : [i32;5] = [1,2,3];		// error
	let arr : [i32;5] = [1,2,3,4,5,6];	// error
	let arr 		  = [1,2,3,4,5,6];  // ok

	// mut
	let     arr1 = [0, 0, 0];
	let mut arr2 = [0, 0, 0];

	arr1[0] = 3; // error
	arr2[0] = 3; // ok
}
