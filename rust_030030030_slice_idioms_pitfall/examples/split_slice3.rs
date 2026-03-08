fn main() {

	let mut a = [9, 8, 7, 6, 5, 4, 3, 2, 1];

	let (s1, s2) = a.split_at_mut(4);

	println!("{s1:?}");
	println!("{s2:?}");

	let (s2, _)  = s2.split_at_mut(4);
	
	println!("{s2:?}");

	s1.sort();
	s2.sort();

	println!("{a:?}");
}
