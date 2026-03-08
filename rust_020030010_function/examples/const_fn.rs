      fn add1(x:usize, y:usize) -> usize { x + y }
const fn add2(x:usize, y:usize) -> usize { x + y }

fn main() {

	let a = 1;
	let b = 1;

	// 배열의 크기는 컴파일 시간에 알아야 한다
	let a1:[i32;add1(1,1)] = [1,2]; // error
	let a2:[i32;add2(1,1)] = [1,2]; // ok
	let a3:[i32;add2(a,b)] = [1,2]; // error

	let ret = add2(a, b); // ok
}