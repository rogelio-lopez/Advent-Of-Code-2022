use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_as_text =
        fs::read_to_string(&args[1]).expect("Failed to load content from input file");

    //This is te problem I think &str type
    let input_vec: Vec<&str> = input_as_text.split('\n').collect();

    let mut total: u32 = 0;
    let mut mid: usize;
    let mut duplicate: char;

    for ruck in input_vec {
        let ruck_vec: Vec<char> = ruck.chars().collect();
        mid = ruck_vec.len() / 2;
        let (left, right) = ruck_vec.split_at(mid);

        duplicate = find_duplicate(left, right);
        total = total + char_ranking(duplicate);
    }

    println!("Total: {}", total);
}

fn find_duplicate(left: &[char], right: &[char]) -> char {
    for i in left {
        for j in right {
            if i == j {
                return *j;
            }
        }
    }
    return ' ';
}

fn char_ranking(letter: char) -> u32 {
    let rank: u32 = letter as u32;

    if letter.is_uppercase() && letter != ' ' {
        return rank - 38;
    } else if !letter.is_uppercase() && letter != ' ' {
        return rank - 96;
    }

    return 0;
}
