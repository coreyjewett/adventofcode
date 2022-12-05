use std::env;
use std::fs;

use phf::phf_map;
use itertools::Itertools;

static SCORE: phf::Map<char, i32> = phf_map! {
    'A' => 1,
    'B' => 2,
    'C' => 3,
    'X' => 1,
    'Y' => 2,
    'Z' => 3,
};

static ROCK:i32 = 1;
static PAPER:i32 = 2;
static SCISSORS:i32 = 3;

static RPS: phf::Map<char, i32> = phf_map! {
    'A' => ROCK,
    'B' => PAPER,
    'C' => SCISSORS,
    'X' => ROCK,
    'Y' => PAPER,
    'Z' => SCISSORS,
};

fn winlosedraw(a: char, b: char) -> i32 {
    let x = RPS.get(&a);
    let y = RPS.get(&b);

    if x == None || y == None { panic!("nope"); }
    if x == y { return 0 };

    let diff = y.unwrap() - x.unwrap();

    // okay scenarios:
    // R - P = -1
    // P - R = 1
    // P - S = -1
    // S - P = 1
    if diff.abs() == 1 { return diff; }

    // broken scenarios
    // S - R = 2
    // R - S = -2
    if diff == 2 { return -1; }
    if diff == -2 { return 1; }

    println!("{} {} {} {}", a, b, x == y, a.eq(&b));
    panic!("incomplet");
}

fn throw(guide: (char, char)) -> i32 {
    return ((winlosedraw(guide.0, guide.1) + 1) * 3)
        + SCORE.get(&guide.1).unwrap();
}

fn tally(guide: Vec<(char, char)>) -> i32 {
    return guide.iter().fold(0, |acc, t| acc + throw(*t) );
}

fn choose(throw: char, guide: char) -> char {
    match guide {
        // throw loser
        'X' => match throw {
            'A' => 'C',
            'B' => 'A',
            'C' => 'B',
            _ => panic!("nah")
        }
        'Y' => throw,
        'Z' => match throw {
            'A' => 'B',
            'B' => 'C',
            'C' => 'A',
            _ => panic!("nah")
        }
        _ => panic!("nah")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let guide: Vec<(char, char)> = contents.trim().lines().map(|l| l.split_whitespace().map(|s| s.chars().nth(0).unwrap() ).collect_tuple().unwrap() ).collect();

    //println!("pt1 {}", tally(guide));
    println!("pt2 {}", tally(guide.iter().map(|t| (t.0, choose(t.0, t.1)) ).collect()));
}
