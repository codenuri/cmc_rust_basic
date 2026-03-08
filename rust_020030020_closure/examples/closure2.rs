fn twice( e : i32 ) -> i32 { e * 2 }

fn main() {

	let k = 3;

	let arr1 = [1, 2, 3];

	let arr2 = arr1.map(twice);

	println!("{arr2:?}"); // 2, 4, 6
}