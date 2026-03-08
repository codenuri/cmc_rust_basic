
fn max<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {	

	if x > y { x } else { y }
}


fn main(){

	let long = 2;
	let r;
	{
		let short = 1;

		r = max(&long, &short);	
	}
	println!("{r}"); // error
}

