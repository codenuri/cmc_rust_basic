fn main()
{	
	let v : i32 = 300;

//	let v1 : i16 = v.into(); // error. i32 => i16

//	let v1 : Result<f32,_> = v.try_into();	// error
	let v2 : Result<i16,_> = v.try_into();	// ok
	let v3 : Result<i8,_>  = v.try_into();	// ok

	println!("{v2:?}");
	println!("{v3:?}");

	let v4 = i16::try_from(v);
	let v5 = i8::try_from(v);

	println!("{v4:?}");
	println!("{v5:?}");
}
