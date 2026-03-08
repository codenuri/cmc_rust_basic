fn ex1() {

}

fn ex2(a : i32, b : i32) -> i32 {
	
	a + b			// 이 코드 와
//	return a + b;	// 이 코드는 동일한 의미
}

fn main() {

	ex1();

	let ret = ex2(1,2);

	println!("{ret}");
}
