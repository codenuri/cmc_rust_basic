fn main() {

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let s : i32 = v
		.iter()
		.filter(|x| *x % 2 == 0)
		.take(3)
		.sum();

	println!("{s}"); // 12


    let out: Vec<String> = v
		.iter()        
		.filter(|x| *x % 2 == 0)                 // 짝수만			: 2, 4, 6, 8, 10
		.skip(1)                                 // 첫 짝수(2) 스킵	 : 4, 6, 8, 10
		.take(2)                                 // 2개만      	    : 4, 6
		.enumerate()                             // (index, value)  :(0, 4), (1, 6)
		.map(|(i, x)| format!("#{}:{}", i, *x))  // 문자열로 변환    : "#0:4", "#1:6"
		.collect();                              // 실행 Trigger. 모든 요소를 Vec<String> 으로

    println!("out = {out:?}"); //  ["#0:4", "#1:6"]
}
