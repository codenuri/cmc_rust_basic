fn main() {
	
	let mut a = [9, 8, 7, 6, 5, 4, 3, 2, 1];

	let (s1, s2) = a.split_at(4);
	let (s1, s2) = a.split_at_mut(4);
	let (s1, s2) = a.split_at_checked(4);
	let (s1, s2) = a.split_at_mut_checked(4);


}
