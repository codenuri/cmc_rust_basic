
fn f1<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {	
	x 
}

fn main(){

	let long = 2;
	let r1;
	let r2;
	{
		let short = 1;

		r1 = f1(&long, &short);	
		r2 = f1(&short, &long);	
	}
	println!("{r1}"); // ok
//	println!("{r2}"); // error
}