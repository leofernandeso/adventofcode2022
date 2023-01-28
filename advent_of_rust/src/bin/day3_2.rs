use std::{collections::{HashMap, HashSet}, hash::Hash};

fn get_item_priority(item: u8) -> u8 {
    if item >= 97 && item <= 122 {
        return item - 96
    }
    return item - (65 - 1) + 26
}

fn main() {
    let mut group_item_counts: HashMap<u8, u8> = HashMap::new();
    let mut priorities_sum: u64 = 0;
    for (i, items) in include_str!("../../../inputs/day03.txt").lines().map(|line| line.as_bytes()).enumerate() {

        let item_set: HashSet<_> = HashSet::from_iter(items.iter());
        item_set.iter().for_each(|item| {
            group_item_counts.entry(**item).and_modify(|item_count| *item_count += 1).or_insert(1);
        });

        if (i + 1) % 3 == 0 {
            let common_group_item = group_item_counts.iter().max_by_key(|entry| entry.1).unwrap();
            priorities_sum += u64::from(get_item_priority(*common_group_item.0));
            group_item_counts.clear();
        }
    }
    println!("{}", priorities_sum);
}