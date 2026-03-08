enum Color1 { Red, Green, Blue, } 	   // 0, 1,  2

enum Color2 { Red = 10, Green = 20, Blue = 30,}

enum Color3 { 
	Red = 10,			// ok
//	Green(i32) = 11, 	// error
}

fn main()
{
//	let n1 : i32 = Color1::Red;			// error
	let n2 : i32 = Color1::Red as i32; 	// ok
	let n3 : i32 = Color2::Red as i32;  // ok

	println!("{} {}", n2, n3);  // 0 10
}
