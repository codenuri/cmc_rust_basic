fn main() {

	for i in 1..10 {
		print!("{i}, "); 	// 1,2,3,4,5,6,7,8,9
	}						
	println!();	


	for i in (1..10).rev() {
		print!("{i}, "); 	// 9,8,7,6,5,4,3,2,1
	}						
	println!();


	for i in (1..10).step_by(2) {
		print!("{i}, "); 	// 1,3,5,7,9
	}
	println!();


	for i in (1..10).take(3) {
		print!("{i}, "); 	// 1,2,3
	}	
	println!();


	for i in (1..10).skip(3) {
		print!("{i}, "); 	// 4,5,6,7,8,9
	}	
	println!();
	

	for i in (1..10).filter( |e| e % 3 == 0 ) {
		print!("{i}, "); 	// 3,6,9
	}
	println!();

	
	for i in (1..10).skip(3).rev().step_by(2) {
		print!("{i}, "); 
	}					// 1..10      : 1,2,3,4,5,6,7,8,9
						// skip(3)    : 4,5,6,7,8,9
						// rev()      : 9,8,7,6,5,4
						// step_by(2) : 9,7,5
	println!();	
}