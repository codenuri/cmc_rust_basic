fn main() 
{
    let ok = false;

    'setup: {
        println!("1) setup start");

        if !ok {
            println!("   setup failed -> exit setup block early");

            break 'setup; 	// 핵심
        }
 		println!("2) setup completed");
    }

    println!("3) continue after setup");
}

