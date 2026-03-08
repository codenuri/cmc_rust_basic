#[derive(Copy, Clone, Debug)]
struct Meter(f64);

#[derive(Copy, Clone, Debug)]
struct Second(f64);

impl std::ops::Deref for Meter {
    type Target = f64;
	fn deref(&self) -> &Self::Target { 	&self.0	}
}

impl std::ops::DerefMut for Meter {   
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 	}
}

impl std::ops::Deref for Second {
    type Target = f64;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl std::ops::DerefMut for Second {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}


fn speed(distance: Meter, time: Second) -> f64 {
    *distance / *time
}

fn main() {

    let meter = Meter(100.0);
    let sec   = Second(9.58);

    println!("Speed: {:.2} m/s", speed(meter, sec)); // ok
//	println!("Speed: {:.2} m/s", speed(sec, meter)); // error
}

