fn main() {

	let s = String::from(" 3.14 ");
//	let s = "3.14";

	let ret = s.trim().parse::<f64>();

	match ret {	
		Ok(v)  => println!("{v}"),
		Err(e) => println!("{e}"),	
	}

	let ret = s.parse::<f64>().unwrap();	
	let ret : f64  = s.parse().unwrap();
	println!("{ret}");

}