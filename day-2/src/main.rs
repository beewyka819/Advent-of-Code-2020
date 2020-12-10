use std::{fs::File, io::Read, path::Path, str::FromStr};

fn main() {
    let input = read_file_to_string("input.txt");
    
    let num_valid = input
        .lines()
        .map(|l| l.split(": "))
        .map(|mut split_line| scan_password_part_2(split_line.next().unwrap(), split_line.next().unwrap()))
        .filter(|b| *b)
        .count();

    println!("There are {} valid passwords", num_valid);
}

fn scan_password_part_1(rules: &str, password: &str) -> bool {
    let mut rules_iter = rules.split(" ");

    let mut min_max = rules_iter
        .next()
        .unwrap()
        .split("-");
    let min = i32::from_str(min_max.next().unwrap()).unwrap();
    let max = i32::from_str(min_max.next().unwrap()).unwrap();
    let constraint = rules_iter
        .next()
        .unwrap()
        .chars()
        .next()
        .unwrap();
    
    let count = password
        .chars()
        .filter(|c| *c == constraint)
        .count() as i32;

    min <= count && count <= max
}

fn scan_password_part_2(rules: &str, password: &str) -> bool {
    let mut rules_iter = rules.split(" ");

    let mut positions = rules_iter
        .next()
        .unwrap()
        .split("-");
    let index_1 = usize::from_str(positions.next().unwrap()).unwrap() - 1;
    let index_2 = usize::from_str(positions.next().unwrap()).unwrap() - 1;
    let constraint = rules_iter
        .next()
        .unwrap()
        .chars()
        .next()
        .unwrap();

    let char_1 = password.chars().nth(index_1);
    let char_2 = password.chars().nth(index_2);

    let mut num_matching = 0;

    if let Some(c) = char_1 {
        num_matching += (c == constraint) as i32;
    }
    if let Some(c) = char_2 {
        num_matching += (c == constraint) as i32;
    }

    num_matching == 1
}

fn read_file_to_string(dir: &str) -> String {
    let path = Path::new(dir);
    let display = path.display();

    let mut f = match File::open(path) {
        Err(why) => panic!("{} not found: {}", display, why),
        Ok(file) => file,
    };

    let mut input_text = String::new();
    f.read_to_string(&mut input_text)
        .expect("Failed to read from file");

    String::from(input_text.trim())
}