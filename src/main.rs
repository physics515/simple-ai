use bigdecimal::BigDecimal;

mod ai;

fn main() {
        std::env::set_var("RUST_BACKTRACE", "full");

	let mut ai = ai::AI::init();
	//println!("ai: {:?}", ai);

	let answer = ai.fire(BigDecimal::from(5_u8));
	println!("input: 5, answer: {}", answer);

        let answer = ai.fire(BigDecimal::from(73_u8));
	println!("input: 73, answer: {}", answer);

	let answer = ai.fire(BigDecimal::from(256_u16));
	println!("input: 256, answer: {}", answer);	

	println!("training...");
	ai.train(training_set());

	let answer = ai.fire(BigDecimal::from(5_u8));
	println!("input: 5, answer: {}", answer);

        let answer = ai.fire(BigDecimal::from(73_u8));
	println!("input: 73, answer: {}", answer);

	let answer = ai.fire(BigDecimal::from(256_u16));
	println!("input: 256, answer: {}", answer);
}

fn training_set() -> Vec<(BigDecimal, BigDecimal)> {
	let mut res = Vec::new();
	for i in 0..10 {
		res.push((BigDecimal::from(i), BigDecimal::from(i * 2)));
	}
	res
}
