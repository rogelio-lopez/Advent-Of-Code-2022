use std::env;
use std::fs;

fn main(){
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(&args[1])
        .expect("Failed to read content for file");

    let number_groups: Vec<&str> = content.split("\n\n").collect();

    let mut largest_num = 0;
    let mut sums_vec: Vec<i32> = Vec::new();
    
    for i in number_groups{
        sums_vec.push(get_sum(i));

        if largest_num < get_sum(i){
            largest_num = get_sum(i);
        }
    }

    //Passing vector by reference
    bubble_sort(&mut sums_vec);

    println!("Largest Num: {}", largest_num);
    println!("vec length: {}", sums_vec[1] + sums_vec[2] + sums_vec[3]); //.len() for length
}

// If mulitple numbers in a group, it returns the sum 
fn get_sum(str_param: &str) -> i32 {
    let mut n = 0;
    let temp_vec: Vec<&str> = str_param.split("\n").collect(); 

    for i in temp_vec{
        if i.len() > 2{
            n = n + i.parse::<i32>().unwrap();
        }
    }
    return n;
}

// Sort vector full of sums 
fn bubble_sort(v: &mut [i32]){
    for i in 0..v.len(){
        for j in 1..v.len(){
            if v[i] > v[j]{
                v.swap(i,j);
            }
        }
    } 
}
