use std::fs;

fn main() {
    let input_file_path = "input.txt";

    println!("read {}", input_file_path);

    let input = fs::read_to_string(input_file_path).expect("Failed to read file!");

    part_two(&input);
}

fn part_one(input: &str) {
    let mut sum = 0;

    for line in input.lines() {
        let mut curr = 0;

        for char in line.chars() {
            match char.to_digit(10) {
                None => {},
                Some(val) => {
                    if curr == 0 {
                        sum += val * 10;
                    }

                    curr = val;
                }
            }
        }

        sum += curr;
    }

    println!("{}", sum);
}

fn part_two(input: &String) {
    let number_words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut sum = 0;

    for line in input.lines() {
        let mut curr = 0;

        for (start, chr) in line.chars().enumerate() {

            match chr.to_digit(10) {
                None => {
                    for (i, &checked_word) in number_words.iter().enumerate() {
                        let i = i as u32;
                        if start + checked_word.len() <= line.len() && line[start..start + checked_word.len()].eq(checked_word) {
                            if curr == 0 {
                                sum += (i + 1) * 10;
                            }
                            curr = i + 1;
                        }
                    }
                },
                Some(val) => {
                    if curr == 0 {
                        sum += val * 10;
                    }

                    curr = val;
                }
            }

        }
        sum += curr;
    }

    println!("{}", sum);
}
