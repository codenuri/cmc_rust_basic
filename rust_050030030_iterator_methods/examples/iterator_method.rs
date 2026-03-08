fn main() {

	let mut v1 = vec![1, 2, 3, 4, 5];
	let mut v2 = vec![1, 2, 3, 4, 5];
	let mut v3 = vec![1, 2, 3, 4, 5];

	let mut it1 = v1.iter();		// Iter<>
	let mut it2 = v2.iter_mut();	// IterMut<>
	let mut it3 = v3.into_iter();	// IntoIter<>

	use_iterator(it1);
	use_iterator(it2);
	use_iterator(it3);
}
fn use_iterator<T:Iterator>(it : T) {

	println!("{}", std::any::type_name_of_val(&it));	

	let s : i32 = it.sum();
	println!("{s}");
}