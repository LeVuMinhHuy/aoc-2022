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
                let first_half = &l[..l.len() / 2];
                let second_half = &l[l.len() / 2..];
                let mut match_characters: Vec<char> = Vec::new();

                for c in second_half.chars() {
                    if first_half.contains(c) && !match_characters.contains(&c) {
                        match_characters.push(c)
                    }
                }

                sum_priority_chars(match_characters.as_slice())
            })
            .sum::<u32>()
    );

    println!(
        "b: {}",
        fs::read_to_string("./input.txt")
            .expect("Should have been able to read the file")
            .split("\n")
            .filter(|&x| !x.is_empty())
            .collect::<Vec<&str>>()
            .chunks(3)
            .map(|l| {
                let first_string = l[0];
                let mut match_characters = Vec::new();

                for c in first_string.chars() {
                    if l[1].contains(c) && l[2].contains(c) && !match_characters.contains(&c) {
                        match_characters.push(c)
                    }
                }

                sum_priority_chars(match_characters.as_slice())
            })
            .sum::<u32>()
    )
}

fn sum_priority_chars(match_characters: &[char]) -> u32 {
    match_characters
        .iter()
        .map(|c| {
            if !c.is_alphabetic() {
                0
            } else {
                if c.is_uppercase() {
                    (*c as u32) - 38u32
                } else {
                    (*c as u32) - 96u32
                }
            }
        })
        .sum::<u32>()
}
