fn main() {

	let score = 80;

	let state = if score > 70 {"pass"} else {"fail"};

	let ret = if score > 70 {5};			// error				
	let ret = if score > 70 {5} else {1.2};	// error
	let ret = if score > 70 {5} else {panic!()}; // ok

	let ret = if score > 70 {5} else {1;};	// error
}
