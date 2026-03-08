use std::time::Instant;

const CNT:usize = 50_000_000;

fn push_no_reserve()
{
	let mut v = Vec::new();

    let start = Instant::now();

    for i in 0..CNT {
        v.push(i);
    }
    let dur = start.elapsed();
    println!("Without reserve: {:?}", dur);
}

fn push_with_reserve()
{
	let mut v = Vec::with_capacity(CNT);

    let start = Instant::now();

    for i in 0..CNT {
        v.push(i);
    }
    let dur = start.elapsed();
    println!("With    reserve: {:?}", dur);
}

fn main() 
{
	push_no_reserve();
	push_with_reserve();
	push_no_reserve();
	push_with_reserve();
}
