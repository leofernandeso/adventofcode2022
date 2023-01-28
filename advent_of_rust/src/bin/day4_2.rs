
fn parse_line(line: &str) -> Vec<u16> {
   let mut parsed_ranges: Vec<u16> = Vec::new();
   for range in line.split(",") {
        let parsed_range: Vec<_> = range.split("-").map(|range_str| range_str.parse::<u16>().unwrap()).collect();
        parsed_ranges.extend(parsed_range);
   }
   parsed_ranges
}

fn verify_range_intersection(rangeA: &[u16], rangeB: &[u16]) -> bool {
    if rangeA[0] <= rangeB[1] && rangeA[1] >= rangeB[0] {
        return true
    }
    if rangeA[0] >= rangeB[1] && rangeA[1] <= rangeB[0] {
        return true
    }
    return false
}

fn main() {
    let mut total_count: u32 = 0;
    for line in include_str!("../../../inputs/day04.txt").lines() {
        let ranges = parse_line(line);
        let intersects = verify_range_intersection(&ranges[0..2], &ranges[2..4]);
        total_count += u32::from(intersects);
    }
    println!("{}", total_count);
}