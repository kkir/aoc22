use std::fs;

fn to_sum(x: &&str) -> u32 {
    x.lines()
        .map(|x| x.parse::<u32>().unwrap())
        .fold(0, |acc, x| acc + x)
}

fn main() {
    let input = fs::read_to_string("input/day1.txt").expect("Must exist");

    let elves: Vec<&str> = input.split("\n\n").collect();

    let mut elves_sum: Vec<u32> = elves.iter().map(to_sum).collect();
    elves_sum.sort_by(|a, b| b.cmp(a));

    let top = elves_sum.first();

    let mut sum_top_three = 0;
    for i in 0..3 {
        sum_top_three += elves_sum[i];
    }

    println!("Top elf: {:?}", top.unwrap());
    println!("Sum top three: {:?}", sum_top_three);
}
