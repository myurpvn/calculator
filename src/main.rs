// #![allow(dead_code)]

#[derive(Debug)]
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
            Self::Division => x / y,
        }
    }
}

fn do_calculations(temp: &mut i32, numbers: &Vec<i32>, ops: &Vec<Operation>) {
    let mut nums_iter = numbers.iter();
    let mut ops_iter = ops.iter();

    let num1 = nums_iter.next().unwrap_or(&0);
    let num2 = nums_iter.next().unwrap_or(&0);

    if let Some(op) = ops_iter.next() {
        *temp = op.calculate(*num1, *num2);
    }
}

fn alter_op_flag(op_flag: &mut bool) {
    *op_flag = !*op_flag;
}
fn main() {
    use std::io::{stdin, stdout, Write};

    println!("Welcome to the rust calculator");

    let mut numbers: Vec<i32> = Vec::new();
    let mut ops: Vec<Operation> = Vec::new();
    let mut result: i32 = 0;
    let mut expect_op: bool = false;

    let mut s = String::new();
    let mut loop_flag = true;

    while loop_flag {
        if expect_op {
            print!("Enter Operation: ");
        } else {
            print!("Enter Value: ");
        }

        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");

        let byte_val = s.trim().as_bytes();

        if expect_op {
            match byte_val {
                b"+" => ops.push(Operation::Addition),
                b"-" => ops.push(Operation::Substraction),
                b"*" => ops.push(Operation::Multiplication),
                b"/" => ops.push(Operation::Division),
                b"=" => {
                    loop_flag = false;
                    println!("Result: {result}");
                }
                _ => {
                    println!("Expecting Operation!");
                    println!("Please enter a valid Operation!");
                    alter_op_flag(&mut expect_op);
                }
            }
        } else {
            use std::str::from_utf8;

            if let Ok(num) = from_utf8(byte_val).unwrap().parse::<i32>() {
                numbers.push(num);
            } else {
                println!("Entered value '{}' is not a valid Number", &s.trim());
                println!("Please enter a valid Number!");
                alter_op_flag(&mut expect_op);
            }
        }

        if numbers.len() == 2 {
            do_calculations(&mut result, &numbers, &ops);
            ops.clear();
            numbers.clear();
            numbers.push(result);
        }

        s.clear();
        alter_op_flag(&mut expect_op);
        // println!("Ops: {ops:?}, Nums: {numbers:?}");
    }

    // println!("Ops: {ops:?}, Nums: {numbers:?}");
}
