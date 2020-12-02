use std::{fs::File, io::Read, path::Path, str::FromStr};

fn main() {
    let input = read_file_to_string("input.txt".to_string());

    let nums = input
        .lines()
        .map(|i| i32::from_str(i).unwrap())
        .collect::<Vec<i32>>();

    let dual_pairs = tuple_pairs_two(&nums);

    let (x, y) = *dual_pairs
        .iter()
        .find(|(x, y)| *x + *y == 2020)
        .expect("No valid combo");

    println!("Part 1: {}", x * y);

    let triple_pairs = tuple_pairs_three(&nums);

    let (x, y, z) = *triple_pairs
        .iter()
        .find(|(x, y, z)| *x + *y + *z == 2020)
        .expect("No valid combo");

    println!("Part 2: {}", x * y * z);
}

fn read_file_to_string<P: AsRef<Path>>(path: P) -> String {
    let mut f = File::open(path).expect("File not found");

    let mut input_text = String::new();
    f.read_to_string(&mut input_text)
        .expect("Failed to read from file");

    return String::from(input_text.trim());
}

fn tuple_pairs_two(nums: &Vec<i32>) -> Vec<(i32, i32)> {
    let mut vec = Vec::new();

    for (i, x) in nums.iter().enumerate() {
        for (j, y) in nums.iter().enumerate() {
            if i != j {
                vec.push((*x, *y));
            }
        }
    }
    vec
}

fn tuple_pairs_three(nums: &Vec<i32>) -> Vec<(i32, i32, i32)> {
    let mut vec = Vec::new();

    for (i, x) in nums.iter().enumerate() {
        for (j, y) in nums.iter().enumerate() {
            for (k, z) in nums.iter().enumerate() {
                if i != j && i != k && j != k {
                    vec.push((*x, *y, *z));
                }
            }
        }
    }

    vec
}
