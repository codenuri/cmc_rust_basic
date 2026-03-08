fn main() {
	let mut a = [1,2,3,4,5];

	// #1. [] 연산으로는 partial borrow 안됨
	let r = &mut a[0];

	let r1 = &a[1];	// error

	println!("{}", *r);

	// #2. ref destructuring pattern 은 
	//     partial borrow 가능
	let [ref mut r1, ref mut r2, ..] = a;

	*r1 = 10;
	*r2 = 20;

	println!("{a:?}");

	// #3. slice
	// => 요소가 아닌 구간(slice)별 reference
	let (r1, r2) = a.split_at_mut(2);

	println!("{r1:?}");	// [10, 20]
	println!("{r2:?}");	// [3, 4, 5]	
}

