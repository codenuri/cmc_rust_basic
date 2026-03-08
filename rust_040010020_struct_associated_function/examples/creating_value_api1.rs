use std::time::Duration;
use std::thread::sleep;

fn main()
{
	let d = Duration::new(5, 500_000_000);	// 5.5 sec
	
	sleep(d); // sleep for 5.5 seconds

	let _d = Duration::default();	 // 0초
	let _d = Duration::from_secs(30); // 30u64 => 
									 //   Duration 30 sec
	let _d = Duration::from_millis(1500);  
	
	let _v = Vec::<i32>::new();
	let _v = Vec::<i32>::with_capacity(100);
}
