fn add1(x : i32, y : i32) -> i32 { x + y }

#[inline]
fn add2(x : i32, y : i32) -> i32 { x + y }

#[inline(always)]
fn add3(x : i32, y : i32) -> i32 { x + y }

#[must_use]
fn add4(x : i32, y : i32) -> i32 { x + y }

fn main()
{
	let _ret1 = add1(1, 2);
	let _ret2 = add3(1, 2);

	add1(1, 2); // ok 
	add2(1, 2); // ok
	add3(1, 2); // ok
	add4(1, 2); // warning
}

