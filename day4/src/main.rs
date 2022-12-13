use std::io::{self, BufRead};

fn extract_nums(v: &[&str]) -> (i32, i32, i32, i32) {
    let v0: Vec<&str> = v[0].split('-').collect();
    let l0 = v0[0].parse::<i32>().unwrap();
    let r0 = v0[1].parse::<i32>().unwrap();
    let v1: Vec<&str> = v[1].split('-').collect();
    let l1 = v1[0].parse::<i32>().unwrap();
    let r1 = v1[1].parse::<i32>().unwrap();
    (l0, r0, l1, r1)
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut counter = 0;
    let mut counter2 = 0;
    for line in lines {
        match line {
            Ok(line) => {
                let v: Vec<&str> = line.split(',').collect();
                let (l0, r0, l1, r1) = extract_nums(&v);
                if (l1 >= l0 && r1 <= r0) || (l1 <= l0 && r0 <= r1) {
                    counter += 1
                }
                if r1 >= l0 && l1 <= r0 {
                    counter2 += 1
                }
            }
            _ => println!("error reading line"),
        }
    }
    println!("counter = {}", counter);
    println!("counter2 = {}", counter2);
}
