fn ex1() {

	let v = vec![1, 2, 3, 4, 5];
	
	let mut it = v.iter();	

	println!("{:?}", it.next()); // "Some(1)"
	println!("{:?}", it.next()); // "Some(2)"
	println!("{:?}", it.next()); // "Some(3)"
	println!("{:?}", it.next()); // "Some(4)"
	println!("{:?}", it.next()); // "Some(5)"
	println!("{:?}", it.next()); // "None"
	println!("{:?}", it.next()); // "None"
}

fn ex2() {

	let v = vec![1,2,3,4,5];
	
	let mut it = v.iter();	

	loop {
		match it.next() {
			Some(e) => print!("{e}, "),
			None => break,
		}	
	}
	println!();
}

fn ex3() {

	let v = vec![1,2,3,4,5];
	
	let mut it = v.iter();	

	while let Some(e) = it.next() {
		print!("{e}, ");
	}
	println!();
}

fn ex4() {

	let v = vec![1,2,3,4,5];
	
	let mut it = v.iter();	

	for e in it {
		print!("{e}, ");
	}
	println!();
}

fn main() {

	ex1();
	ex2();
	ex3();
	ex4();
}
