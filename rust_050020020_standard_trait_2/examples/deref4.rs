struct Meter(f64);

impl std::ops::Deref for Meter {

    type Target = f64;

	fn deref(&self) -> &Self::Target {
		println!("deref()");
		&self.0
	}
}

impl std::ops::DerefMut for Meter {
   
	fn deref_mut(&mut self) -> &mut Self::Target {
		println!("deref_mut()");
		&mut self.0
	}
}


fn main() {

	let mut meter = Meter(100.0);

	let n = *meter;	// let n = *(meter.deref())

	*meter = 123.1; // *(meter.deref_mut()) = 123.1;


	let ret1 = meter.0.powf(1.2);
	let ret2 = meter.powf(1.2);
			// 1. 자신(Meter) 의 메소드 검색
		    // 2. meter.deref().powf(1.2) 메소드 검색 	


	let mut v = vec![1, 2, 3];
	v.sort();	// v.deref_mut().sort()	
				//   ^ &mut [i32] 반환
				// slice 의 메소드인 sort() 를 사용   
}

