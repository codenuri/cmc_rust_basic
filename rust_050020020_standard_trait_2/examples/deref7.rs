#[derive(Debug)]
struct MyVec(Vec<i32>); 

impl std::fmt::Display for MyVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "[{s}]")
    }
}

impl std::ops::Deref for MyVec {
    type Target = Vec<i32>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::ops::DerefMut for MyVec {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}




fn main() {
    let mut v = MyVec(vec![3, 1, 2]);

	v.push(4);	// v.deref_mut().push(4)
				// Vec<i32> 메소드 

	v.sort(); 	// v.deref_mut().deref_mut().sort()
				// &mut [i32] 메소드

    println!("{}",   v); // [1, 2, 3, 4]
	println!("{:?}", v); // [1, 2, 3, 4]
}