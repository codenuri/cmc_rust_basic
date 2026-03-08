fn main()
{
	let n : i32 = 3;

	let r1 : &i32 = &n; 
	let r2 : &i32 = &3;

	let r3 = &n; 
	let r4 = &3;

	let a = r1;	
	let b = *r1;

	let ret = (*r1).pow(2);
	let ret = r1.pow(2);
	let ret = *r1.pow(2);	// error
}

