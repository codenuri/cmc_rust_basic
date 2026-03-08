#![allow(unused)]

fn main()
{	
	let f : f32 = 3.4;
	let d : i32 = 3;
	
	let v1 : f64 = f.into(); // f32 => f64  ok
	let v2 : i32 = f.into(); // f32 => i32  error

	let v3 : i64 = d.into(); // i32 => i64  ok 
	let v4 : i16 = d.into(); // i32 => i16  error
	let v5 : u32 = d.into(); // i32 => u32  error  
}

