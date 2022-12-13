use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines();
    let mut carry = 0;
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    for line in lines {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    if carry > first {
                        third = second;
                        second = first;
                        first = carry;
                    } else if carry > second {
                        third = second;
                        second = carry;
                    } else if carry > third {
                        third = carry;
                    }
                    carry = 0;
                } else {
                    match line.parse::<i32>() {
                        Ok(v) => carry += v,
                        _ => println!("error converting {} to integer", line),
                    }
                }
            }
            _ => println!("error reading line"),
        }
    }
    println!("{},{},{}={}", first, second, third, first + second + third);
}
