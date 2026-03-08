fn DoSomthing() -> Option<i32> { Some(30) }

fn ex1() {

	let v;	// [핵심] 이 변수가 필요

	match DoSomthing() {
		Some(e) => { v = e }, 
		None => return,
	}

	// v 를 사용한 코드 작성
}

fn ex2() {
	let v = match DoSomthing() {
				Some(e) => e,
				None => return,
			};
	
	// v 를 사용한 코드 작성
}

fn ex3() {
	
	let Some(v) = DoSomthing() else { return };
	
	// v 를 사용한 코드 작성
}


fn main() {
	ex1();
	ex2();
	ex3();
}

let v = match DoSomthing() {
			Some(e) => e,
			None => return,
		};
	
// v 를 사용한 코드 작성