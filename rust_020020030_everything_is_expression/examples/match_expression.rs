fn main() {

	let score = 85;
	let grade;
	match score
	{
		90..=100 => grade = 'A',
		80..=89 =>  grade = 'B',
		_       =>  grade = 'C'
	}
	
	let grade = match score 
				{
					90..=100 => 'A',
					80..=89  => 'B',
					_ => 'C'
				};

	let grade = make_grade(score);
}

fn make_grade(score:i32) -> char {

	match score 
	{
		90..=100 => 'A',
		80..=89  => 'B',
		_ => 'C'
	}

}

fn main() {

	let grade = make_grade(score);
}