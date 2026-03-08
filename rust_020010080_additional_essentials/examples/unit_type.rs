fn f1()       { }		
fn f2() -> () { }	

fn main()
{
	let v : () = ();

	let ret = f1();

	println!("{ret:?}");
}
