use std::collections::HashSet;

fn get_item_priority(item: u8) -> u8 {
    if item >= 97 && item <= 122 {
        return item - 96
    }
    return item - (65 - 1) + 26
}

fn main() {
    
    let mut total: u32 = 0;
    for line in include_str!("../../../inputs/day03.txt").lines().map(|line| line.as_bytes()) {
        let middle_point = line.len() / 2;
        let first_compartment: HashSet<&u8> = HashSet::from_iter(line[0..middle_point].iter());
        let second_compartment: HashSet<&u8> = HashSet::from_iter(line[middle_point..].iter());
        let items_priorities_sum: u32 = first_compartment
            .intersection(&second_compartment)
            .map(|item|u32::from(get_item_priority(**item)))
            .sum();
        total += items_priorities_sum;
    }
    println!("{}", total);
}