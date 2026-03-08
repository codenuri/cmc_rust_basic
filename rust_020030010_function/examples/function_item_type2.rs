fn add(a : i32, b : i32) -> i32 {
	a + b
}

fn sub(a : i32, b : i32) -> i32 {
	a + b
}

fn main() {

	let mut f1 = add;
	let mut f2 = add as fn(i32, i32)->i32;
		
	f1 = sub; // error. 
	f2 = sub; // ok	    
}
