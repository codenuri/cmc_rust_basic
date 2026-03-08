fn main() {

	let s1 = String::from("ABCD");
	let s2 = String::from("ABCD");
	let n1 = 10;
	
	let t = s1;			// move
  	let t = s2.clone(); // deep copy
	let t = n1;			// copy Copy Type

//	println!("{s1}");	// error
	println!("{s2}");	// ok
	println!("{n1}");	// ok

	//------------------------------
	fn f1(s : String)  		{}
	fn f2(s : &String) 		{}
	fn f3(s : String) -> String	{ s } 
	
//	f1(s); 			// move	
	f1(s.clone());	// deep copy
	f2(&s);			
	let s = f3(s);
	//------------------------------
	let mut v = Vec::<String>::new();

//	v.push(s);			// move
	v.push(s.clone());	// deep copy
	println!("{}", s);  // move 아님
}
