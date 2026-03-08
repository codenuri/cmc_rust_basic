fn my_split_at_mut<T>(s: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    	
    let ptr = s.as_mut_ptr();
    let len = s.len();

	let left;
	let right;

	unsafe {
    	left  = std::slice::from_raw_parts_mut(ptr, mid);
    	right = std::slice::from_raw_parts_mut(ptr.add(mid), len - mid);
	}
    (left, right)
}

fn main() {

	let mut a = [9, 8, 7, 6, 5, 4, 3, 2, 1];

	let (s1, s2) = my_split_at_mut(&mut a, 4);

	println!("{s1:?}");
	println!("{s2:?}");

	s1.sort();
	s2.sort();

	println!("{a:?}");	
}