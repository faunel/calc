
use std::io;
use std::process;

enum MathOperations {
	Sum(f64, f64),
	Sub(f64, f64),
	Mul(f64, f64),
	Div(f64, f64),
	Rem(f64, f64),
}

#[derive(Debug)]
enum MathErrors{
	DivZero
}

impl MathOperations {
	fn calc(&self) -> Result<f64, MathErrors>{
		match &self {
			&MathOperations::Sum(a, b) => Ok(a + b),
			&MathOperations::Sub(a, b) => Ok(a - b),
			&MathOperations::Mul(a, b) => Ok(a * b),
			&MathOperations::Div(a, b) => {
				if *b == 0.0 {
					Err(MathErrors::DivZero)
				} else {
					Ok(a / b)
				}
			},
			&MathOperations::Rem(a, b) => Ok(a % b),
		}
	}

	fn show (mo: Result<f64, MathErrors>, sum:f64, digit:f64, symbol: String) -> f64{
		match mo {
			Ok(res) => {
				println!("{} {} {} = {}", sum, symbol, digit, res);
				res
			},
			Err(e) => match e {
				MathErrors::DivZero => {
					println!("Помилка ділення на нуль");
					println!("{}", sum);
					sum
				}
			},
		}
	}
}

fn main() {
    println!("Введіть перше число");
	let mut sum = read_to_float();

	loop {
		let mut symbol = String::new();
        println!("Введіть символ операції (+, -, *, х, /, ÷, %, =)");
		io::stdin().read_line(&mut symbol).unwrap();
		let symbol = symbol.trim();

		match symbol {
			"+" => {
				println!("Введіть число яке потрібно додати до \"{}\".", sum);
				let digit = read_to_float();
				let mo = MathOperations::Sum(sum, digit).calc();
				sum = MathOperations::show(mo, sum, digit, symbol.to_owned());
			}

			"-" => {
				println!("Введіть число яке потрібно відняти від \"{}\".", sum);
				let digit = read_to_float();
				let mo = MathOperations::Sub(sum, digit).calc();
				sum = MathOperations::show(mo, sum, digit, symbol.to_owned());
			}
            "*" | "Х" | "х" |"x" | "X" => {
				println!("Введіть число яке потрібно помножити на \"{}\".", sum);
				let digit = read_to_float();
				let mo = MathOperations::Mul(sum, digit).calc();
				sum = MathOperations::show(mo, sum, digit, symbol.to_owned());
			}
			"/" | "÷" => {
				println!("Введіть число яке потрібно на яке потрібно поділити \"{}\".", sum);
				let digit = read_to_float();
				let mo = MathOperations::Div(sum, digit).calc();
				sum = MathOperations::show(mo, sum, digit, symbol.to_owned());
			}
			"%" => {
				println!("Введіть число для якого потрібно визначити залишок від ділення до  \"{}\".", sum);
				let digit = read_to_float();
				let mo = MathOperations::Rem(sum, digit).calc();
				sum = MathOperations::show(mo, sum, digit, symbol.to_owned());
			}
			"=" => {
				println!("Фінальне значення \"{}\".", sum);
				process::exit(0);
			}
			&_ => {
				println!("Не правильний символ");
			}
		}
	}
}

fn read_to_float() -> f64 {
	let num: f64;
	loop {
		let mut number = String::new();
		io::stdin().read_line(&mut number).unwrap();
		let number = number.trim().parse::<f64>();
		num = match number{
			Ok(n) => n,
			Err(_) => {
				println!("Ви ввели не число");
				continue;
			},
		};
		break;
	};
	num
}
