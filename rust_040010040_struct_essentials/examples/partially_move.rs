#[derive(Debug)]
struct Person {
	name : String,
	age  : u8,
}	

fn main() {

	let p = Person{name:String::from("alice"), age:10};

	let _t = p;	// p 자체가 move

//	println!("{:?}", p);	  // error
//	println!("{:?}", p.name); // error
//	println!("{:?}", p.age);  // error

	//-----------------------------------------------
	let p = Person{name:String::from("alice"), age:10};

	let _s = p.name; // partially move

//	println!("{:?}", p);	  // error
//	println!("{:?}", p.name); // error
	println!("{:?}", p.age);  // ok

	//-----------------------------------------------
	let p = Person{name:String::from("alice"), age:10};

	let Person { name, age } = p; // partially move

//	println!("{:?}", p);	  // error
//	println!("{:?}", p.name); // error
	println!("{:?}", p.age);  // ok

	//-------------------------------------------------
	let p = Person{name:String::from("alice"), age:10};

	let Person { ref name, age } = p; 

	println!("{:?}", p);	  // ok
	println!("{:?}", p.name); // ok
	println!("{:?}", p.age);  // ok
	
}

