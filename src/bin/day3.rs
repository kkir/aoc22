use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::Deref,
};

fn find_common_item(tuple: (&str, &str)) -> Option<char> {
    let (one, two) = tuple;
    let mut items = HashSet::new();
    for x in one.chars() {
        items.insert(x);
    }
    for x in two.chars() {
        if items.contains(&x) {
            return Some(x);
        }
    }
    return None;
}

fn distance(ch1: char, ch2: char) -> u32 {
    ch2 as u32 - ch1 as u32
}

fn get_priority(x: Option<char>) -> u32 {
    let char = x.unwrap();
    match char {
        'a'..='z' => 1 + distance('a', char),
        'A'..='Z' => 27 + distance('A', char),
        _ => 0,
    }
}

fn get_common_item(group: Vec<&str>) -> Option<char> {
    let mut items: HashMap<char, u8> = HashMap::new();
    let mut found = false;
    let mut sets = vec![];
    for elf in group {
        let chars: Vec<char> = elf.chars().collect();
        let set: HashSet<char> = HashSet::from_iter(chars.iter().cloned());
        sets.push(set)
    }
    let a: HashSet<char> = &sets[0] & &sets[1];
    let b: HashSet<char> = &a & &sets[2];
    let first = b.iter().map(|x| *x).nth(0);
    return first;
}

fn main() {
    let input = fs::read_to_string("input/day3.txt").expect("Must exist");
    let result: u32 = input
        .lines()
        .map(|x| x.split_at(x.len() / 2))
        .map(find_common_item)
        .map(get_priority)
        .sum();

    let mut i = 0;
    let mut group: Vec<&str> = vec![];
    let mut items: Vec<Option<char>> = vec![];
    for line in input.lines() {
        group.push(line);
        i += 1;
        if i % 3 == 0 {
            items.push(get_common_item(group));
            group = vec![];
        }
    }
    println!("{:?}", items);
    let result2: u32 = items.iter().map(|x| get_priority(*x)).sum();

    println!("{}", result2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = get_common_item(vec![
            "NGvdqJmJvpNbGRMGQgRsfgfn",
            "WlHTHShlLwSWjFRsncfbcwsgQc",
            "BHtSBHWHSCWLZHlhjTHLLdbNNqNpzpDzNvDvtPmmPp",
        ]);
        assert_eq!(result, Some('b'));
    }
}
