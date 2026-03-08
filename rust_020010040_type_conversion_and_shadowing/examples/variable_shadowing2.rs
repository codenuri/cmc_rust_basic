fn main()
{	
	// ex1
	let age = "30";
	let age : i32 = age.parse().unwrap();

	
	// ex2
	let mut value : i32 = 0;

	// value 를 사용한 복잡한 연산
	// 연산중 value 는 변경될수 있고
	// 연산후 최종 결과는 value 에 있다.
	
	// 계산 종료 후에는 immutable 한 상태로 value 사용
	let value = value;
}
