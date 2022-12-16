use std::{fs, io};
use std::num::ParseIntError;

fn main() {
    let result = read_file().expect("Error reading file");
    let split_text = result.lines();
    let mut array: Vec<i32> = vec![0, 0, 0];

    let mut current_calories = 0;
    for item in split_text {
        match get_number(item.to_string()) {
            Ok(i) => {
                current_calories += i;
            },
            Err(_e) => {
                array = add_to_top_three(array, current_calories);
                current_calories = 0;
            }
        }
    }

    array = add_to_top_three(array, current_calories);
    let mut total = 0;
    for item in array {
        total += item;
    }
    println!("Total is :{}", total);
}

fn add_to_top_three(mut top_three: Vec<i32>, item_to_add: i32) -> Vec<i32> {
    let mut should_be_in_top_three = false;
    for current_item in top_three.iter_mut() {
        if item_to_add > *current_item {
            should_be_in_top_three = true;
        }
    }
    if should_be_in_top_three {
        top_three.push(item_to_add);
        top_three.sort();
        top_three.reverse();
        top_three.pop();
        println!("Top 3 {:?}", top_three);
    }
    top_three
}

fn get_number(item: String) -> Result<i32, ParseIntError> {
    item.parse::<i32>()
}

fn read_file() -> io::Result<String>{
    let input = fs::read_to_string("src/day-1/input.txt")?;
    Ok(input)
}