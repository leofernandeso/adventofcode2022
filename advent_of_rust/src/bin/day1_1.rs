fn main() {
    let lines: Vec<_> = include_str!("../../../inputs/day01.txt").lines().collect();
    let biggest = lines
        .split(|line| line.is_empty())
        .map(|group| {
            group
                .iter()
                .map(|val| val.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .max().unwrap();
    println!("{}", biggest);
}