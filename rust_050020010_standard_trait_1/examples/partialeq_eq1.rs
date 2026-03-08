fn check_partial_eq<T:PartialEq>(arg : T) { }

fn check_eq<T:Eq>(arg : T) {}

fn main() {

	let f1 = 3.1111111111111111111111111111111112;
	let f2 = 3.1111111111111111111111111111111113;

	println!("{}", f1 == f2 );	// true
	
	check_partial_eq(1);	// ok
	check_partial_eq(1.1);	// ok
	check_eq(1); 			// ok
	check_eq(1.1);			// error
}
