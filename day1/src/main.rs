use std::fs;

fn main() {
    //find elf with most calories
    let file_string = fs::read_to_string("text.txt").unwrap();
    let mut max_value = 0;
    let mut max_elf = 0;
    let mut sum = 0;
    let mut cur_elf = 0;
    for line in file_string.lines() {
        if line.len() <= 0 {
            if sum > max_value {
                max_value = sum;
                max_elf = cur_elf as i32 + 1;
            }
            cur_elf += 1;
            sum = 0;
            continue;
        }
        sum += line.parse::<i32>().unwrap();
    }
    println!("{max_elf}:{max_value}");
}
