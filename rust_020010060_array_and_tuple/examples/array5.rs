fn main() {

	let arr = [1, 3, 5, 2, 4];

	println!("{}", 	 arr.len());			
	println!("{}", 	 arr.contains(&2));
	println!("{}", 	 arr.is_empty());
	println!("{:?}", arr.first()); 

	let mut arr = arr.map(|e| e * 2);
	println!("{arr:?}");

	arr.sort(); 
	println!("{arr:?}");

	arr.sort_by( |a, b| b.cmp(a) );
	println!("{arr:?}");

	arr[1..4].sort();
	println!("{arr:?}");

	arr.reverse();
	println!("{arr:?}");
}
