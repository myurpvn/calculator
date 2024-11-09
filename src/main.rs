#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Addition,
    Substraction,
    Multiplication,
    Division,
}

impl Operation {
    fn calculate(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Addition => x + y,
            Self::Substraction => x - y,
            Self::Multiplication => x * y,
            Self::Division => {
                if y == 0 {
                    -1
                } else {
                    x / y
                }
            }
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
enum Input {
    Number(i32),
    Operation(Operation),
    Unsupported,
}

fn calculate_input(input_vec: &Vec<Input>) -> i32 {
    let mut result_vec: Vec<i32> = Vec::new();
    let mut ops_vec: Vec<&Operation> = Vec::new();

    for input in input_vec {
        match input {
            Input::Number(num) => result_vec.push(*num),
            Input::Operation(operation) => ops_vec.push(operation),
            _ => {}
        }
        if result_vec.len() == 2 {
            let num2 = result_vec.pop().unwrap();
            let num1 = result_vec.pop().unwrap();
            let ops = ops_vec.pop().unwrap();

            if ops == &Operation::Division && num2 == 0 {
                println!("Zero Division Encountered");
            }
            let r = ops.calculate(num1, num2);
            result_vec.push(r);
        }
    }

    result_vec.pop().unwrap()
}

fn split_input(s: &str) -> (bool, Vec<Input>) {
    let mut splitted_input: Vec<Input> = Vec::new();
    let mut num_string: String = String::new();
    let mut is_valid = true;

    let mut s_iter = s.chars(); // "55+5+6"
    let mut expect_num = true;

    while let Some(c) = s_iter.next() {
        if c.is_numeric() {
            num_string.push(c);
        } else {
            if num_string.len() > 0 {
                if expect_num {
                    splitted_input.push(Input::Number(num_string.parse().unwrap()));
                    expect_num = false;
                    num_string.clear();
                } else {
                    is_valid = false;
                    break;
                }
            }
            let input_ops: Input = match c {
                '+' => Input::Operation(Operation::Addition),
                '-' => Input::Operation(Operation::Substraction),
                '*' => Input::Operation(Operation::Multiplication),
                '/' => Input::Operation(Operation::Division),
                _ => Input::Unsupported,
            };

            if !expect_num {
                splitted_input.push(input_ops);
                expect_num = true;
            } else {
                is_valid = false;
                break;
            }
        }
    }

    if num_string.len() > 0 && expect_num {
        splitted_input.push(Input::Number(num_string.parse().unwrap()));
    } else {
        is_valid = false;
    }

    (is_valid, splitted_input)
}

fn main() {
    use std::io::{stdin, stdout, Write};
    println!("============ Welcome to the Rust Calculator ============");

    loop {
        print!("Enter Expression: ");

        let mut s: String = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");

        s = s.trim().to_string();

        if s.chars().next().unwrap_or('_') == 'q' {
            break;
        }

        let (is_valid, splitted_input) = split_input(&s);

        if is_valid {
            let result = calculate_input(&splitted_input);
            println!("Result: {result}\n");
        } else {
            println!("Input is not valid!");
            println!("Input: {s}");
        }
    }

    println!("Bye :)")
}
