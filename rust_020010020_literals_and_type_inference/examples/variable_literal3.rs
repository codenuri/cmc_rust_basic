#![allow(unused)]

fn main()
{
	// binary, octal, dec, hex 
	let n1 = 10;	// 10진수
	let n2 = 0b10;	// 2진수
	let n3 = 0o10;	// 8진수
	let n4 = 0x10;	// 16진수

	// literal suffix
	let v1 : i32 = 10;	// 32bit signed   integer
	let v2 : u32 = 10;	// 32bit unsigned integer
	
	let v3 = 10i32;	
	let v4 = 10u32;
	let v5 = 10_u32;	

	let v6 = 3.4f64;	// f64 type
	let v7 = 3.4f32;	// f32 type
	let v8 = 0x10u8;	// u8  type 

	// digit separator
	let a1 = 1_000_000;
	let a2 = 0b1010_1001;
	let f2 = 3.141_592;
}
	