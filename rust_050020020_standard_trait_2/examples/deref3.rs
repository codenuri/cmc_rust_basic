struct Meter(f64);

fn main() {

	let meter = Meter(100.0);

	println!("{}", meter.0);	

	let ret1 = meter.0.powf(2.1); // ok

	let ret2 = meter.powf(2.1);   // error

}
