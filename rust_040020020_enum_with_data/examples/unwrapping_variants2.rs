#[derive(Debug, Copy, Clone)]
enum MyOption<T> {
	Some(T),
	None,
}
impl<T> MyOption<T> {
	fn unwrap(self) -> T {
		match self {
			MyOption::Some(v) => v,
			MyOption::None => panic!(),
		}
	}
}
fn main() {
	let opt : MyOption<i32> = MyOption::Some(10);

	let value = opt.unwrap();

	println!("{value}");
}