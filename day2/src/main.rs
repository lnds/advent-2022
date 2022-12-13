use std::io::{self, BufRead};

fn defeats(o: &str, p: &str) -> bool {
    match o {
        "A" => p == "Z",
        "B" => p == "X",
        "C" => p == "Y",
        "X" => p == "C",
        "Y" => p == "A",
        "Z" => p == "B",
        _ => false,
    }
}

fn base_score(p: &str) -> i32 {
    match p {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}

fn find_lose(o: &str) -> &str {
    match o {
        "A" => "Z",
        "B" => "X",
        "C" => "Y",
        _ => "",
    }
}

fn find_win(o: &str) -> &str {
    match o {
        "A" => "Y",
        "B" => "Z",
        "C" => "X",
        _ => "",
    }
}

fn find_draw(o: &str) -> &str {
    match o {
        "A" => "X",
        "B" => "Y",
        "C" => "Z",
        _ => "",
    }
}

fn score(o: &str, p: &str) -> i32 {
    if defeats(o, p) {
        base_score(p)
    } else if defeats(p, o) {
        base_score(p) + 6
    } else {
        base_score(p) + 3
    }
}

fn score2(o: &str, p: &str) -> i32 {
    match p {
        "X" => base_score(find_lose(o)),
        "Y" => base_score(find_draw(o)) + 3,
        "Z" => base_score(find_win(o)) + 6,
        _ => 0,
    }
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut total = 0;
    let mut total2 = 0;
    for line in lines {
        match line {
            Ok(line) => {
                let v: Vec<&str> = line.split(' ').collect();
                total += score(v[0], v[1]);
                total2 += score2(v[0], v[1])
            }
            _ => println!("error reading line"),
        }
    }
    println!("total = {}", total);
    println!("total2 = {}", total2);
}
