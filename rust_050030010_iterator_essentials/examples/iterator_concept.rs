use std::collections::LinkedList;

fn main() {

    let s = LinkedList::from([1, 2, 3, 4, 5]);
	let v = vec![1, 2, 3, 4, 5];
	
	let mut it1 = s.iter();
	let mut it2 = v.iter();

	while let Some(e) = it1.next() {
		print!("{e}, ");
	}
	println!();

	while let Some(e) = it2.next() {
		print!("{e}, ");
	}
	println!();
}
