use std::iter::zip;

pub fn get_distance() -> Result<i32, anyhow::Error> {
    let mut array1: Vec<i32> = Vec::new();
    let mut array2: Vec<i32> = Vec::new();

    let input = include_str!("./input.txt");
    for line in input.lines() {
        let mut split_number = line.split_whitespace();
        array1.push(split_number.next().unwrap().parse::<i32>().unwrap());
        array2.push(split_number.next().unwrap().parse::<i32>().unwrap());
    }

    array1.sort();
    array2.sort();

    // zip() is a function that takes two iterators and combines them into a single iterator of tuple.
    let distance: i32 = zip(array1, array2).map(|x| (x.0 - x.1).abs()).sum();

    Ok(distance)
}
