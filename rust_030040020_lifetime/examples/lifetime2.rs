// 아래 코드는 컴파일 안됨
// 개념 설명을 위한 예제
fn foo(...) -> &i32 {
	// ...
}

fn main() {

	let	r = foo(...); // foo 함수가 반환한 Reference 의
					  // 대상의 수명은 어떻게 될까 ?
	
			
	println!("{r}");	
}
