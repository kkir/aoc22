use std::fs;

#[derive(PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum Game {
    Win,
    Draw,
    Lose,
}

fn get_points((a, b): &(char, char)) -> i32 {
    let my = match b {
        'X' => Hand::Rock,
        'Y' => Hand::Paper,
        _ => Hand::Scissors,
    };
    let opponent = match a {
        'A' => Hand::Rock,
        'B' => Hand::Paper,
        _ => Hand::Scissors,
    };
    let hand_points = match my {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
        _ => 0,
    };

    let game = if my == opponent {
        Game::Draw
    } else {
        match (my, opponent) {
            (Hand::Rock, Hand::Scissors) => Game::Win,
            (Hand::Paper, Hand::Rock) => Game::Win,
            (Hand::Scissors, Hand::Paper) => Game::Win,
            _ => Game::Lose,
        }
    };

    let game_points = match game {
        Game::Win => 6,
        Game::Draw => 3,
        Game::Lose => 0,
    };

    return hand_points + game_points;
}

fn win_hand(opponent: Hand) -> Hand {
    match opponent {
        Hand::Rock => Hand::Paper,
        Hand::Paper => Hand::Scissors,
        Hand::Scissors => Hand::Rock,
    }
}

fn lose_hand(opponent: Hand) -> Hand {
    match opponent {
        Hand::Rock => Hand::Scissors,
        Hand::Paper => Hand::Rock,
        Hand::Scissors => Hand::Paper,
    }
}

fn get_points2((a, b): &(char, char)) -> i32 {
    let my_game = match b {
        'X' => Game::Lose,
        'Y' => Game::Draw,
        _ => Game::Win,
    };
    let opponent = match a {
        'A' => Hand::Rock,
        'B' => Hand::Paper,
        _ => Hand::Scissors,
    };

    let my_hand = match my_game {
        Game::Win => win_hand(opponent),
        Game::Draw => opponent,
        Game::Lose => lose_hand(opponent),
    };

    let hand_points = match my_hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
        _ => 0,
    };

    let game_points = match my_game {
        Game::Win => 6,
        Game::Draw => 3,
        Game::Lose => 0,
    };

    return hand_points + game_points;
}

fn main() {
    let input = fs::read_to_string("input/day2.txt").expect("Must exist");
    let strategy: Vec<(char, char)> = input
        .lines()
        .map(|x| (x.chars().nth(0).unwrap(), x.chars().nth(2).unwrap()))
        .collect();
    let result = strategy.iter().fold(0, |acc, x| acc + get_points2(x));
    println!("{:?}", result);
}
