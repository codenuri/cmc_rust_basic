enum Color1 { Red, Green, Blue, } 	// 최대값 2 

enum Color2 { 	  // 최대값 300
	Red   = 100, 
	Green = 200, 
	Blue  = 300,		
} 

#[repr(i32)]
enum Color3 { Red, Green, Blue, }	// 최대값 2 	 

fn main()
{
	let c1 = Color1::Blue;
	let c2 = Color2::Blue;
	let c3 = Color3::Blue;

	println!("{}", std::mem::size_of_val(&c1) ); // 1
	println!("{}", std::mem::size_of_val(&c2) ); // 2
	println!("{}", std::mem::size_of_val(&c3) ); // 4
}
