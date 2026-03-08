#[derive(Copy, Clone, Debug)]
struct Point {
	x : i32,
	y : i32,
}

fn main() {

	// 배열 값으로 Vec 생성
	let v = Vec::<i32>::from([1,2,3]); 


	// tuple 값으로 Point 생성 
	let p = Point::from((1, 2)); // ?? 
								 
}