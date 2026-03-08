fn main() {

	let mut v = vec![1, 2, 3];

	// #1. 정보 얻기
	println!("{}", v.len());		// 3
	println!("{}", v.capacity());	// 3
	println!("{}", v.is_empty());	// false

	// #2. 요소 추가/삭제
	v.push(4);			// => 1, 2, 3, 4
	v.insert(1, 10);	// => 1, 10, 2, 3, 4
    v.pop();			// => 1, 10, 2, 3
	v.remove(1);		// => 1, 2, 3
	v.extend([9,8,7]);  // => 1, 2, 3, 9, 8, 7

	println!("{v:?}");	// 1, 2, 3, 9, 8, 7

	// #3. 정렬
	v.sort(); 					 // => 1, 2, 3, 7, 8, 9
	v.sort_by( |a, b| b.cmp(a)); // => 9, 8, 7, 3, 2, 1

	println!("{v:?}");	// 9, 8, 7, 3, 2, 1

	// #4. closure 를 인자로 활용
	v.retain( |e| e % 2 == 0 ); 	// 짝수만 남기고 제거
	println!( "{v:?}" );			// 8, 2
}