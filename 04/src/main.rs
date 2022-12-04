use std::fs;

fn main() {
    println!("Advent of Code 2022");
    println!("Day 3: https://adventofcode.com/2022/day/3");
    println!("");

    println!(
        "a: {}",
        fs::read_to_string("./input.txt")
            .expect("Should have been able to read the file")
            .split("\n")
            .filter(|&x| !x.is_empty())
            .map(|l| {
                if let [first, second] = l.split(",").collect::<Vec<&str>>()[..] {
                    let x = first.split("-").collect::<Vec<&str>>();
                    let y = second.split("-").collect::<Vec<&str>>();

                    let a = x[0].parse::<u32>().unwrap();
                    let b = x[1].parse::<u32>().unwrap();
                    let c = y[0].parse::<u32>().unwrap();
                    let d = y[1].parse::<u32>().unwrap();

                    if (a <= c && b >= d) || (a >= c && b <= d) {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
            .filter(|x| *x == true)
            .count()
    );

    println!(
        "b: {}",
        fs::read_to_string("./input.txt")
            .expect("Should have been able to read the file")
            .split("\n")
            .filter(|&x| !x.is_empty())
            .map(|l| {
                if let [first, second] = l.split(",").collect::<Vec<&str>>()[..] {
                    let x = first.split("-").collect::<Vec<&str>>();
                    let y = second.split("-").collect::<Vec<&str>>();

                    let a = x[0].parse::<u32>().unwrap();
                    let b = x[1].parse::<u32>().unwrap();
                    let c = y[0].parse::<u32>().unwrap();
                    let d = y[1].parse::<u32>().unwrap();

                    if d < a || c > b {
                        false
                    } else {
                        true
                    }
                } else {
                    false
                }
            })
            .filter(|x| *x == true)
            .count()
    );
}
