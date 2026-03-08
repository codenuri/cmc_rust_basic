fn main() {

	let n1 = {3};	
	let n2 = {3;};  

	println!("{n1:?}"); // 3
	println!("{n2:?}"); // ()

	let n3 = { let k = 3;
			   k * 2
			 };

	println!("{n3:?}"); // 6

	let _ret1 = f1();
	let _ret2 = f2();
}

fn f1() -> i32 {

	return 10; 
}

fn f2() -> i32 {
	
	10 
}