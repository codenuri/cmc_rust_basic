fn main() {

	let mut opt = Some(30);	// Option 타입
//	opt = None;

	let Some(v) = opt else { panic!() };

	println!("{v}"); // 30
}
