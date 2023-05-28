use std::io::{self, BufRead, BufReader, Write};

struct Calculator {
    num1: f64,
    num2: f64,
}

#[derive(PartialEq, Debug)]
enum Errors {
    DivideZero,
    RemainingZero,
    Overflow,
}

impl Calculator {
    fn new(num1: f64, num2: f64) -> Calculator {
        Calculator { num1, num2 }
    }

    fn add(&self) -> Result<f64, Errors> {
        let one = self.num1 as i128;
        let two = self.num2 as i128;

        if one.checked_add(two).is_none() {
            Err(Errors::Overflow)
        } else {
            Ok(self.num1 + self.num2)
        }
    }

    fn subtract(&self) -> Result<f64, Errors> {
        let one = self.num1 as i128;
        let two = self.num2 as i128;

        if one.checked_sub(two).is_none() {
            Err(Errors::Overflow)
        } else {
            Ok(self.num1 - self.num2)
        }
    }

    fn multiply(&self) -> Result<f64, Errors> {
        let one = self.num1 as i128;
        let two = self.num2 as i128;

        if one.checked_mul(two).is_none() {
            Err(Errors::Overflow)
        } else {
            Ok(self.num1 * self.num2)
        }
    }

    fn divide(&self) -> Result<f64, Errors> {
        let one = self.num1 as i128;
        let two = self.num2 as i128;

        if self.num2 == 0.0 {
            Err(Errors::DivideZero)
        } else if one.checked_div(two).is_none() {
            Err(Errors::Overflow)
        } else {
            Ok(self.num1 / self.num2)
        }
    }

    fn remaining(&self) -> Result<f64, Errors> {
        let one = self.num1 as i128;
        let two = self.num2 as i128;

        if self.num2 == 0.0 {
            Err(Errors::RemainingZero)
        } else if one.checked_rem(two).is_none() {
            Err(Errors::DivideZero)
        } else {
            Ok(self.num1 % self.num2)
        }
    }

    fn check_errors(err: Errors) {
        match err {
            Errors::DivideZero => println!("Помилка переповнення"),
            Errors::RemainingZero => println!("Помилка отримання залишку від ділення на нуль"),
            Errors::Overflow => println!("Помилка переповнення"),
        };
    }
}

fn main() {
    loop {
        println!("Введіть ваш вираз через пробіли (Приклад, 2 + 3):");
        let mut input = String::new();

        io::stdout().flush().unwrap(); // Очистка вихідного буфера

        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin);

        match reader.read_line(&mut input) {
            Ok(_) => {
                let trimmed_input = input.trim();

                if trimmed_input.eq_ignore_ascii_case("exit") {
                    println!("завершення програми...");
                    break;
                }

                let parts: Vec<&str> = trimmed_input.split_whitespace().collect();

                if parts.len() != 3 {
                    println!("Недійсний вираз");
                    continue;
                }

                let num1: f64 = match parts[0].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Ви ввели \"{}\". Очікувалось число", parts[0]);
                        continue;
                    }
                };

                let operator: char = match parts[1].parse() {
                    Ok(op) => op,
                    Err(e) => {
                        println!("Помилка {}", e);
                        continue;
                    }
                };

                let num2: f64 = match parts[2].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Ви ввели \"{}\". Очікувалось число", parts[2]);
                        continue;
                    }
                };

                let calculator = Calculator::new(num1, num2);

                let result = match operator {
                    '+' => match calculator.add() {
                        Ok(result) => result,
                        Err(err) => {
                            Calculator::check_errors(err);
                            continue;
                        }
                    },
                    '-' => match calculator.subtract() {
                        Ok(result) => result,
                        Err(err) => {
                            Calculator::check_errors(err);
                            continue;
                        }
                    },
                    '*' => match calculator.multiply() {
                        Ok(result) => result,
                        Err(err) => {
                            Calculator::check_errors(err);
                            continue;
                        }
                    },
                    '/' => match calculator.divide() {
                        Ok(result) => result,
                        Err(err) => {
                            Calculator::check_errors(err);
                            continue;
                        }
                    },
                    '%' => match calculator.remaining() {
                        Ok(result) => result,
                        Err(err) => {
                            Calculator::check_errors(err);
                            continue;
                        }
                    },
                    _ => {
                        println!(
                            "Не правильний оператор \"{}\". Очікувались наступні (+, -, /, *)",
                            operator
                        );
                        continue;
                    }
                };

                println!("{} {} {} = {}", num1, operator, num2, result);
            }
            Err(error) => {
                println!("Помилка: {}", error);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(Calculator::new(2.0, 3.0).add(), Ok(5.0));
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(Calculator::new(8.0, 6.0).subtract(), Ok(2.0));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(Calculator::new(8.0, 6.0).multiply(), Ok(48.0));
    }

    #[test]
    fn test_division() {
        assert_eq!(Calculator::new(25.0, 5.0).divide(), Ok(5.0));
    }

    #[test]
    fn test_remaining() {
        assert_eq!(Calculator::new(11.0, 3.0).remaining(), Ok(2.0));
    }

    #[test]
    fn test_division_by_zero() {
        assert_eq!(Calculator::new(5.0, 0.0).divide(), Err(Errors::DivideZero));
    }

    #[test]
    fn test_remaining_by_zero() {
        assert_eq!(
            Calculator::new(5.0, 0.0).remaining(),
            Err(Errors::RemainingZero)
        );
    }

    #[test]
    fn test_overflow_multiply() {
        assert_eq!(
            Calculator::new(std::f64::MAX, 2.0).multiply(),
            Err(Errors::Overflow)
        );
    }

    #[test]
    fn test_overflow_add() {
        assert_eq!(
            Calculator::new(std::f64::MAX, 2.0).add(),
            Err(Errors::Overflow)
        );
    }
}
