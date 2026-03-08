fn main() {

	let mut v = vec![1, 2, 3, 4, 5];

	println!("{:?}", v.iter().sum::<i32>());     // 15
	println!("{:?}", v.iter().product::<i32>()); // 120
	println!("{:?}", v.iter().fold(10, |s, e| s + e ) ); // 25  s = 10; s = s + e
	println!("{:?}", v.iter().min() ); // Some(1)

	println!("{:?}", v.iter().find(|e| *e % 2 == 0) ); 	// Some(2)  e : &&i32
	println!("{:?}", v.iter().all( |e| e % 2 == 0)); 	// false    e : &i32
	println!("{:?}", v.iter().any( |e| e % 2 == 0)); 	// true		e : &i32

	
	let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	
	let mut v = a.into_iter().collect::<Vec<i32>>();

	v.iter_mut().for_each( |e| if *e % 2 == 0 { *e = 0;});

	println!("{:?}", v);	// [1, 0, 3, 0, 5, 0, 7, 0, 9, 0]

	let (v1, v2) : (Vec<i32>, Vec<i32>)= v.into_iter().partition(|e| *e == 0 );
	println!("{v1:?}");	// [0, 0, 0, 0, 0]
	println!("{v2:?}"); // [1, 3, 5, 7, 9]
}

