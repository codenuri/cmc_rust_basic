struct MyFilter<I, P> {
    iter: I,
    pred: P,
}
impl<I, P> Iterator for MyFilter<I, P>
where  I: Iterator, P: FnMut(&I::Item) -> bool, 
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.iter.next() {
            if (self.pred)(&x) {
                return Some(x);
            }
        }
        None
    }
}

fn main() {

    let v = vec![1, 2, 3, 4, 5];

	let mut it1 = v.iter();
	let mut it2 = it1.filter(|e| *e % 2 == 0);

	let mut it1 = v.iter();
	let mut it2 = MyFilter{ iter: it1, 
							pred: |e:&&i32| *e % 2 == 0 };

	while let Some(e) = it2.next() {
		print!("{e}, ");	// 2, 4
	}
	println!();

	let mut it = v.iter().rev().take(5).filter(|e| *e % 2 == 0);

	let ret = it.next(); // filter_iter.next() =>
		   // take_iter.next() => rev_iter.next() => iter().next()

}
