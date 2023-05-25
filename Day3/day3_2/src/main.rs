use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_as_text: String = fs::read_to_string(&args[1]).expect("Failed to read from file");

    let input_vec: Vec<&str> = input_as_text.split('\n').collect();

    let mut total: u32 = 0;

    for i in (0..(input_vec.len() - 3)).step_by(3) {
        let v_1: Vec<char> = input_vec[i].chars().collect();
        let v_2: Vec<char> = input_vec[i + 1].chars().collect();
        let v_3: Vec<char> = input_vec[i + 2].chars().collect();

        total = total + find_key_val(v_1, v_2, v_3);
    }

    println!("Total: {}", total);
}

fn find_key_val(v_1: Vec<char>, v_2: Vec<char>, v_3: Vec<char>) -> u32 {
    let max_len: usize = if v_2.len() >= v_3.len() {
        v_2.len()
    } else {
        v_3.len()
    };

    for i in v_1 {
        let mut v2_found: bool = false;
        let mut v3_found: bool = false;

        for j in 0..(max_len) {
            if v_2.get(j) != None {
                if v_2[j] == i {
                    v2_found = true;

                    if v2_found && v3_found {
                        return get_letter_score(v_2[j]);
                    }
                }
            }
            if v_3.get(j) != None {
                if v_3[j] == i {
                    v3_found = true;

                    if v2_found && v3_found {
                        return get_letter_score(v_3[j]);
                    }
                }
            }
        }
    }
    return 0;
}

fn get_letter_score(letter: char) -> u32 {
    let char_as_num: u32 = letter as u32;

    if letter.is_uppercase() && letter != ' ' {
        return char_as_num - 38;
    } else if !letter.is_uppercase() && letter != ' ' {
        return char_as_num - 96;
    }
    return 0;
}
