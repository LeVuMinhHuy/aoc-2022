use std::fs;

fn main() {
    println!("Advent of Code 2022");
    println!("Day 1: https://adventofcode.com/2022/day/1");
    println!("");

    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let inventories: Vec<&str> = input.split("\n").collect();
    let mut total_calories_list: Vec<u32> = Vec::new();

    let mut sum: u32 = 0;
    for inventory in inventories {
        if !inventory.is_empty() {
            sum += inventory.parse::<u32>().unwrap();
        } else {
            total_calories_list.push(sum);
            sum = 0;
        }
    }

    println!("Number of Elves: {}", total_calories_list.len());

    if !(total_calories_list.len() > 0) {
        panic!("You did something wrong!");
    }

    let mut max: u32 = total_calories_list[0];

    for calories in &total_calories_list {
        if *calories > max {
            max = *calories
        }
    }

    println!("Max calories: {max}");

    bubble_sort(total_calories_list.as_mut_slice());

    if !(total_calories_list.len() > 2) {
        panic!("You did something wrong!");
    }

    println!(
        "Top three calories: {}, {}, {}",
        total_calories_list[0], total_calories_list[1], total_calories_list[2]
    );

    println!(
        "Total top three calories: {}",
        total_calories_list[0] + total_calories_list[1] + total_calories_list[2]
    );
}

fn bubble_sort(list: &mut [u32]) {
    let length = list.len();
    for i in 0..length {
        for j in 0..(length - i - 1) {
            if list[j] < list[j + 1] {
                let temp = list[j];
                list[j] = list[j + 1];
                list[j + 1] = temp;
            }
        }
    }
}
