use std::{fs, ops::RangeInclusive};

fn to_range(s: &str) -> RangeInclusive<u32> {
    let mut iter = s.split('-').map(|s| s.parse::<u32>().unwrap());
    return iter.next().unwrap()..=iter.next().unwrap();
}

fn contain(ranges: Vec<RangeInclusive<u32>>) -> u32 {
    let r1 = ranges[0].clone();
    let r2 = ranges[1].clone();
    return if r1.contains(r2.start()) && r1.contains(r2.end())
        || (r2.contains(r1.start()) && r2.contains(r1.end()))
    {
        1
    } else {
        0
    };
}

fn overlap(ranges: Vec<RangeInclusive<u32>>) -> u32 {
    let r1 = ranges[0].clone();
    let r2 = ranges[1].clone();
    return if r1.contains(r2.start())
        || r1.contains(r2.end())
        || (r2.contains(r1.start()) || r2.contains(r1.end()))
    {
        1
    } else {
        0
    };
}

fn main() {
    let input = fs::read_to_string("input/day4.txt").expect("Must exist");
    let result = input
        .lines()
        .map(|line| contain(line.split(',').map(to_range).collect()))
        .sum::<u32>();

    let result2 = input
        .lines()
        .map(|line| overlap(line.split(',').map(to_range).collect()))
        .sum::<u32>();

    println!("{}", result2.to_string())
}
