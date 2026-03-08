fn f1() { loop {} }			// -> ()
fn f2() -> ! { loop {} }

fn main() {

	let s = 75;

	let g = if s > 60 { "PASS" } else { f1() };	// error
	let g = if s > 60 { "PASS" } else { f2() };	// ok


	let v : i32 = return;		// compile ok
	let v : f64 = panic!();		// compile ok
	let v : String = loop{};	// compile ok	
}
