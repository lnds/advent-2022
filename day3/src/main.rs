use std::io::{self, BufRead};
use std::collections::HashSet;

fn split_line(line: &str) -> (&str, &str) {
    let len = line.len();
    (&line[0..len/2], &line[len/2..])
}

fn intersect(a:&str, b:&str) -> HashSet<u8> {
    let (shorter, longer) = if a.len() > b.len() {
        (b, a)
    }  else {
        (a, b)
    };
    let set: HashSet<char> = shorter.chars().collect();
    longer.chars().filter(|c| set.contains(&c)).map(|c| c as u8).collect()
}

fn intersect3(a:&str, b:&str, c: &str) -> HashSet<u8> {
    let set: HashSet<char> = a.chars().collect();
    let set2: HashSet<char>  = b.chars().filter(|c| set.contains(&c)).collect();
    c.chars().filter(|c| set2.contains(&c)).map(|c| c as u8).collect()
}

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut score = 0;
    let mut score2 = 0;
    let mut count = 0;
    let mut triplet: [String; 3] = Default::default();
    while let Some(line) = lines.next() {
        match line {
            Ok(line) => {
                let (left, right) = split_line(&line);
                let common = intersect(left, right);
                triplet[count] = line.to_string();
                score += common.iter().map(|&c| if c >= b'a' && c <= b'z' { ((c - b'a') as i32) + 1 } else { ((c - b'A') as i32) + 27}).sum::<i32>()
            }
            _ => println!("error reading line")
        }
        count += 1;
        if count == 3 {
            let common = intersect3(&triplet[0], &triplet[1], &triplet[2]);
            score2 += common.iter().map(|&c| if c >= b'a' && c <= b'z' { ((c - b'a') as i32) + 1 } else { ((c - b'A') as i32) + 27}).sum::<i32>();
            count = 0;
        }

    }
    println!("score = {}", score);
    println!("score2 = {}", score2)
}

