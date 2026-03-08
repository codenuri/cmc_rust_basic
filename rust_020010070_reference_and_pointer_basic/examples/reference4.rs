fn f1(_a : i32)  { } // 인자가 reference 아님
fn f2(_a : &i32) { } // 인자가 reference 

fn main()
{
	let n = 10;

	f1(n);
	f1(10);
	f2(&n);
	f2(&10);

	let arr = [1i32, 2, 3, 4, 5];

//	println!("{}", arr.contains(2)); // error
	println!("{}", arr.contains(&2)); // ok
}
