fn main()
{
	let score = 85;

	match score {
		80..90 => println!("B"),
		_ => {}
	}

	if let 80..90 = score { 
		println!("B");
	}

	if matches!(score, 80..90 ) {
		println!("B");
	}	

	if score >= 80 && score < 90 {
		println!("B");
	}	

	if (80..90).contains(&score) {
		println!("B");
	}	
}
