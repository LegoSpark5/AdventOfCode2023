use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path: &str = "./src/input.txt";
    let nums_as_str: [&str; 20] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
                                    "zero", "one", "two", "three", "four",
                                    "five", "six", "seven", "eight", "nine"];

    let mut total: usize = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            let num = 
                    what_is_first(&line, &nums_as_str) * 10 +
                    what_is_last(&line, &nums_as_str);
            total = total + num;
        }
    }

    println!("The total is {total}!");
}

fn what_is_first (input: &str, check_vals: &[&str] ) -> usize {
    let mut first_loc: usize = usize::MAX;
    let mut output: usize = 0;
    for (i, val) in check_vals.iter().enumerate() {
        if let Some(current_loc) = input.find(val) {
            if current_loc < first_loc {
                first_loc = current_loc;
                output = i % 10;
            }
        }
    }
    output
}

fn what_is_last (input: &str, check_vals: &[&str] ) -> usize {
    let mut last_loc: usize = 0;
    let mut output: usize = 0;
    for (i, val) in check_vals.iter().enumerate() {
        if let Some(current_loc) = input.rfind(val) {
            if current_loc >= last_loc {
                last_loc = current_loc;
                output = i % 10;
            }
        }
    }
    output
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}