use std::fs;

fn main() {
    let nums_as_chars: std::ops::Range<char> = '0'..':';
    let file_path: &str = "./src/input.txt";
    let mut ones: u32 = 0;
    let mut tens: u32 = 0;
    let mut total: u32 = 0;
    let mut flag: bool = true;

    let input: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    for next_char in input.chars() {
        if next_char == '\n' {
            total = total + ones + tens * 10;
            flag = true;
        } else if nums_as_chars.contains(&next_char) {
            ones = next_char as u32 - 0x30;
            if flag {
                tens = next_char as u32 - 0x30;
                flag = false;
            }
        }
    }
    println!("The total is {total}!");
}
