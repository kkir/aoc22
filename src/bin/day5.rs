use std::{fs, num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.split(" ").collect::<Vec<&str>>();
        return Ok(Move {
            amount: chars[1].parse::<usize>().unwrap(),
            from: chars[3].parse::<usize>().unwrap(),
            to: chars[5].parse::<usize>().unwrap(),
        });
    }
}

fn main() {
    let input = fs::read_to_string("input/day5.txt").unwrap();
    let mut iter = input.split("\n\n");
    let mut crates = iter.next().unwrap().lines().rev();
    let n = crates
        .next()
        .unwrap()
        .split("   ")
        .map(|x| x.trim())
        .collect::<Vec<&str>>()
        .len();
    let mut boxes = (0..n).map(|_x| vec![]).collect::<Vec<Vec<char>>>();
    for row in crates {
        for i in 0..n {
            let j = 1 + 4 * i;
            let char = row.chars().nth(j).unwrap();
            if char != ' ' {
                boxes[i].push(char);
            }
        }
    }
    let moves: Vec<Move> = iter
        .next()
        .unwrap()
        .lines()
        .map(|x| Move::from_str(x).unwrap())
        .collect();

    for m in moves {
        // part 1
        // for _i in 0..m.amount {
        //     let val = boxes[m.from - 1].pop().unwrap();
        //     boxes[m.to - 1].push(val);
        // }
        let from = &mut boxes[m.from - 1];
        let len = from.len();
        let drained = from.drain(len - m.amount..).collect::<Vec<char>>();
        for c in drained.iter() {
            boxes[m.to - 1].push(*c);
        }
    }

    let str = boxes
        .iter()
        .fold(vec![] as Vec<char>, |acc, b| match b.last() {
            Some(&x) => [acc, [x].to_vec()].concat(),
            None => acc,
        });
    print!("{:?}", String::from_iter(str));
}
