use std::env;
use std::fs;

// fn fuel_grow(crabs: Vec<i32>, pos: i32) -> i32 {
//     println!("Fuel: {} {:?}", pos, crabs);
//     return crabs.iter().fold(0, |accum, hpos| {
//         println!("  {} {}", accum, hpos);
//         println!("  {}", (1..(hpos-pos).abs()).fold(accum, |fuel, n| {
//             println!("    {}\t{} {} {}", accum, fuel, n, fuel + n);
//             return fuel + n
//         }));
//         return (0..(hpos-pos).abs()).fold(accum, |fuel, n| fuel + n);
//     });
// }

// fn opt_dist(crabs: Vec<i32>) -> i32 {
//     let crabst = (0..crabs.len()).rev().map(|i| (i, fuel_grow(crabs.clone(), i as i32)));
//     crabst.clone().for_each(|t| println!("{:?}", t));

//     return crabst.reduce(|a,b| if a.1 < b.1 { a } else { b })
//         .unwrap().0.try_into().unwrap();
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let input_nums: Vec<Vec<i32>> = contents.trim().split("\n\n").map(|s| s.split('\n').map(|s| s.parse().unwrap()).collect() ).collect();

    let mut sums: Vec<i32> = input_nums.iter().map(|i| i.iter().sum() ).collect();
    sums.sort();
    sums.reverse();

    // println!("{}", sums[0]);

    println!("{}", &sums[0..3].iter().sum::<i32>());

    // println!("{}", fuel_grow(input_nums.clone(), 2));
    // println!("{}", opt_dist(input_nums.clone()));
    // println!("{}", fuel_grow(input_nums.clone(), opt_dist(input_nums)));
}
