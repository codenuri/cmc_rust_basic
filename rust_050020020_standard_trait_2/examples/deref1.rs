
fn speed(distance: f64, time: f64) -> f64 {
    distance / time
}

fn main() {

    let meter = 100.0;
    let sec   = 9.58;

    let sp1 = speed(meter, sec);
	
    println!("Speed: {:.2} m/s", sp1);

	let sp2 = speed(sec, meter); // 인자를 잘못 전달 
						// 컴파일 에러가 있을까 ? 에러 아님
						// 핵심 : distance 와 time 은 같은 타입이므로 

	println!("Speed: {:.2} m/s", sp2);
}
