use std::time::Instant;

const CNT : i32 = 1000;	// 루프 횟수

// 대략 1MB 크기 문자열 2개 반환
fn make_large_string() -> (String, String) {

	("a".repeat(1_000_000), "b".repeat(1_000_000))
}

fn move_swap() {
    let (mut a, mut b) = make_large_string();    

    let start = Instant::now();	// 현재 시간 기록

    for _ in 0..CNT {
        let tmp = a;	// move 발생
        a = b;
        b = tmp;
    }

    let elapsed = start.elapsed();	// 경과된 시간 얻기
    println!("move    swap : {:.2?}", elapsed);
}

fn clone_swap() {
	let (mut a, mut b) = make_large_string(); 

    let start = Instant::now();

    for _ in 0..CNT {
        let tmp = a.clone();
        a = b.clone();
        b = tmp.clone();
    }

    let elapsed = start.elapsed();
    println!("clone() swap : {:.2?}", elapsed);
}

fn main() {
	move_swap();
    clone_swap();    	
}
