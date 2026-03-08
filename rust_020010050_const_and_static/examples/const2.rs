fn main() {

	const SIZE : usize = 32; // const
	let   size : usize = 32; // immutable binding
	
//	SIZE = 10; // error	
//	size = 10; // error

	let v1 = SIZE;	
	let v2 = size;

	let arr1 : [i32;SIZE]; // 
	let arr2 : [i32;size]; //  

	let n = 3;
	let   c1 : i32 = 3; // ?
	let   c2 : i32 = n; // ?
	const C1 : i32 = 3; // ?
	const C2 : i32 = n; // ?
}
