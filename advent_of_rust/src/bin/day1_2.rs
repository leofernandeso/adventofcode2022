fn main() {
    let lines: Vec<_> = include_str!("../../../inputs/day01.txt").lines().collect();
    let mut calories_per_sack: Vec<u64> = lines
        .split(|line| line.is_empty())
        .map(|group| {
            group
                .iter()
                .map(|val| val.parse::<u64>().unwrap())
                .sum::<u64>()
        }).collect();
    calories_per_sack.sort_by(|a, b| b.cmp(a));
    let sum_of_first_three: u64 = calories_per_sack[0..3].iter().sum();
    println!("{:?}", sum_of_first_three);
}