#![allow(unused)]

fn main()
{	
	let f : f32 = 3.4;
	
	let v1 : f64 = f.into(); 	 // f32 -> f64
	let v2 : f64 = f64::from(f); // 위와 동일 의미


	let v3 = f as f64;		// ok
	let v4 = f64::from(f);	// ok
	let v5 = f.into();		// error
}
