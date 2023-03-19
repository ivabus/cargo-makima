#![allow(clippy::let_and_return)]

use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use serde::Deserialize;

const RESPONSES: &str = include_str!("../responses.json");

#[derive(Deserialize)]
struct Responses {
	positive: Vec<String>,
	negative: Vec<String>,
}

enum ResponseType {
	Positive,
	Negative,
}

fn main() {
	let code = real_main().unwrap_or_else(|e| {
		eprintln!("Error: {:?}", e);
		-1
	});
	std::process::exit(code)
}

fn real_main() -> Result<i32, Box<dyn std::error::Error>> {
	let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
	let mut arg_iter = std::env::args();
	let _cargo = arg_iter.next();

	let mut cmd = std::process::Command::new(cargo);
	cmd.args(arg_iter);
	let status = cmd.status()?;
	eprintln!("\x1b[1m");
	if status.success() {
		eprintln!("{}", select_response(ResponseType::Positive))
	} else {
		eprintln!("{}", select_response(ResponseType::Negative));
	}
	eprintln!("\x1b[0m");
	Ok(status.code().unwrap_or(-1))
}

fn select_response(response_type: ResponseType) -> String {
	let mut rng = StdRng::from_entropy();

	// Choose what mommy will say~
	let responses: Responses = serde_json::from_str(RESPONSES).expect("RESPONSES to be valid JSON");

	let response = match response_type {
		ResponseType::Positive => &responses.positive,
		ResponseType::Negative => &responses.negative,
	}
	.choose(&mut rng)
	.expect("non-zero amount of responses");
	// Done~!
	response.clone()
}

fn parse_options(env_var: &str, default: &str) -> Vec<String> {
	std::env::var(env_var)
		.unwrap_or_else(|_| default.to_owned())
		.split('/')
		.map(|s| s.to_owned())
		.collect()
}
