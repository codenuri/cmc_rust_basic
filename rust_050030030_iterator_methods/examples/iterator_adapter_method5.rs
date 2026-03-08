fn main() {

	let a = [10, 20, 30];
    let b = [1, 2, 3, 4];

    let mixed: Vec<i32> = a
        .iter()
        .copied()
        .chain(b.iter().copied()) // [10,20,30] 뒤에 [1,2,3,4] 연결
        .rev()                    // 뒤집기
        .step_by(2)               // 2칸씩
        .collect();

    println!("mixed = {mixed:?}");

    let pairs: Vec<(i32, i32)> = [1, 2, 3]
        .iter()
        .copied()
        .zip([10, 20, 30, 40].iter().copied()) // 짧은 쪽 길이에 맞춤
        .collect();

    println!("pairs = {pairs:?}");

    let flat: Vec<i32> = [1, 2, 3]
        .iter()
        .copied()
        .flat_map(|x| [x, x * 10]) // 각 원소를 2개로 펼침
        .collect();

    println!("flat = {flat:?}");
}