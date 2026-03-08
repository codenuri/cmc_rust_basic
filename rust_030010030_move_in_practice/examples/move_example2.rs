use std::any::type_name_of_val;

fn main() {
	let s = String::from("alice");
	
	match s	{
		// let x = s
		a => println!("{}", type_name_of_val(&a)),		
		_ => {}		
	}

	let s = String::from("alice");
	match &s	{
		a => println!("{}", type_name_of_val(&a)),		
		_ => {}		
	}

	match s	{
		// let ref x = s
		ref a => println!("{}", type_name_of_val(&a)),	
		_ => {}						
	}
	// ---------------------------------------
	let v = vec![1,2,3,4,5];

//	for e in v        { // v 는 move 됨
//	for e in &v 	  { // v 는 move 안됨
	for e in v.iter() { // v 는 move 안됨						
	}
	
}
			
	

