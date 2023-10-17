use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_content: String = fs::read_to_string(&args[1]).expect("Failed to load file");

    let assignments: Vec<&str> = file_content
        .split("\n")
        .filter(|n| n != &"\n" && n != &"")
        .collect();

    //let mut total_contains: u32 = 0;
    //will be comparing in doubles so "in input_vec.step_by(2)"
    for i in (0..assignments.len()).step_by(2) {
        let mut first_elf_nums: Vec<&str> = assignments[i].split("  ").collect();
        first_elf_nums = first_elf_nums[1].split("-").collect();

        //   let mut second_elf_nums: Vec<&str> = assignments[i + 1].split("  ").collect();
        //   second_elf_nums = second_elf_nums[1].split("-").collect();

        println!("{:?}", convert_to_string(first_elf_nums));
    }
}

//Create vector then turn to string from starting to finish number
fn convert_to_string(elf_nums: Vec<&str>) -> String {
    let first_num: i32 = elf_nums[0].parse().unwrap();
    let second_num: i32 = elf_nums[1].parse().unwrap();

    let mut str_vec: String = format!("{}", elf_nums[0]);

    for i in (first_num + 1)..(second_num + 1) {
        let temp = str_vec;
        str_vec = format!("{}, {}", temp, i);
    }

    return str_vec;
}
