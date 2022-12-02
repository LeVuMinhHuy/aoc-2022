use std::fs;

fn main() {
    println!("Advent of Code 2022");
    println!("Day 2: https://adventofcode.com/2022/day/2");
    println!("");

    let input = fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let rounds: Vec<&str> = input.split("\n").collect();
    let mut points: Vec<u32> = Vec::new();
    let mut sneak_points: Vec<u32> = Vec::new();

    for round in rounds {
        if !round.is_empty() {
            let shapes: Vec<&str> = round.split(' ').collect::<Vec<&str>>();

            if shapes.len() < 2 {
                panic!("You did something wrong");
            };

            points.push(rules(shapes[0], shapes[1]));
            sneak_points.push(sneak_rules(shapes[0], shapes[1]));
        }
    }

    println!("a: {}", points.iter().sum::<u32>());
    println!("b: {}", sneak_points.iter().sum::<u32>());
}

// X: Rock, Y: Paper, Z: Scissor
// A: Rock, B: Paper, C: Scissor

fn rules(opponent: &str, me: &str) -> u32 {
    match me {
        "X" => match opponent {
            "A" => 1 + 3,
            "B" => 1,
            "C" => 1 + 6,
            _ => panic!("You did something wrong"),
        },
        "Y" => match opponent {
            "A" => 2 + 6,
            "B" => 2 + 3,
            "C" => 2,
            _ => panic!("You did something wrong"),
        },
        "Z" => match opponent {
            "A" => 3,
            "B" => 3 + 6,
            "C" => 3 + 3,
            _ => panic!("You did something wrong"),
        },
        _ => panic!("You did something wrong"),
    }
}

// X: Lose, Y: Draw, Z: Win
// A: Rock - 1, B: Paper - 2, C: Scissor - 3

fn sneak_rules(opponent: &str, result: &str) -> u32 {
    match result {
        "X" => match opponent {
            "A" => 0 + 3,
            "B" => 0 + 1,
            "C" => 0 + 2,
            _ => panic!("You did something wrong"),
        },
        "Y" => match opponent {
            "A" => 3 + 1,
            "B" => 3 + 2,
            "C" => 3 + 3,
            _ => panic!("You did something wrong"),
        },
        "Z" => match opponent {
            "A" => 6 + 2,
            "B" => 6 + 3,
            "C" => 6 + 1,
            _ => panic!("You did something wrong"),
        },
        _ => panic!("You did something wrong"),
    }
}
