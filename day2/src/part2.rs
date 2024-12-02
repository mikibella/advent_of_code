pub fn process(input: &str) -> usize {
    let mut counter: usize = 0;
    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        if is_sorted_and_step_not_bigger_than_three(&numbers) {
            counter += 1;
            continue;
        }
        if check_with_removing_one_level(numbers) {
            counter += 1;
            continue;
        }
    }
    counter
}

fn check_with_removing_one_level(numbers: Vec<i32>) -> bool {
    //skip over each element of the vec before checking
    for i in 0..numbers.len() {
        let mut new_numbers = numbers.clone();
        new_numbers.remove(i);
        if is_sorted_and_step_not_bigger_than_three(&new_numbers) {
            return true;
        }
    }
    false
}

fn is_sorted_and_step_not_bigger_than_three(vec: &[i32]) -> bool {
    let is_increasing = vec.windows(2).all(|w| w[0] < w[1]);
    let is_decreasing = vec.windows(2).all(|w| w[0] > w[1]);
    let step_not_bigger_than_three = vec.windows(2).all(|w| (w[0] - w[1]).abs() <= 3);

    (is_increasing || is_decreasing) && step_not_bigger_than_three
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(process(input), 4);
    }
}
