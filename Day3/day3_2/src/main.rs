use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_as_text: String = fs::read_to_string(&args[1]).expect("Failed to read from file");

    let input_vec: Vec<&str> = input_as_text.split('\n').collect();

    for i in (0..(input_vec.len() - 3)).step_by(3) {
        let mut v_1: Vec<char> = input_vec[i].chars().collect();
        let v_2: Vec<char> = input_vec[i + 1].chars().collect();
        let v_3: Vec<char> = input_vec[i + 2].chars().collect();

        v_1.extend(v_2);
        v_1.extend(v_3);

        order_vec(v_1);
    }
}

fn order_vec(unordered_vec: Vec<char>) -> Vec<char> {
    let mut ordered_vec: Vec<char> = unordered_vec;

    for i in 0..(ordered_vec.len() - 1) {
        for j in 1..ordered_vec.len() {
            if ordered_vec[i] > ordered_vec[j] {
                let temp: char = ordered_vec[i];
                ordered_vec[i] = ordered_vec[j];
                ordered_vec[j] = temp;
            }
        }
    }

    return ordered_vec;
}

fn find_key_val(vect: Vec<char>) -> u32 {
    if vect.len() > 3 {
        for i in 0..(vect.len() - 3) {
            if vect[i] == vect[i + 1] && vect[i + 1] == vect[i + 2] {
                return 1;
            }
        }
    }

    return 0;
}

//fn char_to_dec(letter: char) -> u32{}
